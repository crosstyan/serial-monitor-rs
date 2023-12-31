use crate::serial::api::out as api;
use crate::serial::api::out::serial_service_server as service;
use flume;
use flume::{Receiver, Sender};
use rand::Rng;
use serialport::SerialPort;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::vec::Vec;
use std::{collections::HashMap, pin::Pin};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::UdpSocket;
use tokio::sync::{Mutex, RwLock};
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tracing::{debug, error, info, instrument, trace, warn};

// a workaround for SerialStream not being Send
pub struct SyncSerialStream(pub SerialStream);

impl Deref for SyncSerialStream {
    type Target = SerialStream;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SyncSerialStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// https://stackoverflow.com/questions/68704717/is-the-sync-trait-a-strict-subset-of-the-send-trait-what-implements-sync-withou
unsafe impl Sync for SyncSerialStream {}

pub struct Channel<T> {
    pub rx: Arc<Receiver<T>>,
    pub tx: Arc<Sender<T>>,
}

pub type BufferType = Vec<u8>;
pub type PinnedSerialPort = Pin<Arc<Mutex<SyncSerialStream>>>;
pub struct ManagedSerialDevice {
    port: PinnedSerialPort,
    port_name: String,
    options: api::ManagedOptions,
    udp: Option<Arc<Mutex<UdpSocket>>>,
    outbound_handle: tokio::task::JoinHandle<()>,
    inbound_handle: tokio::task::JoinHandle<()>,
    /// outbound refers to data going from the serial port to the outside world.
    /// [Reader] i.e. `rx` is expected to be used. Don't touch `tx`.
    outbound: Channel<BufferType>,
    /// inbound refers to data coming from the outside world to the serial port
    /// [Writer] i.e. `tx` is expected to be used. Don't touch `rx`.
    inbound: Channel<BufferType>,
}

impl ManagedSerialDevice {
    pub fn outbound(&self) -> Arc<Receiver<BufferType>> {
        self.outbound.rx.clone()
    }
    pub fn inbound(&self) -> Arc<Sender<BufferType>> {
        self.inbound.tx.clone()
    }
    pub fn port_name(&self) -> &str {
        &self.port_name
    }
    pub fn udp(&self) -> Option<Arc<Mutex<UdpSocket>>> {
        self.udp
    }
    pub fn options(&self) -> &api::ManagedOptions {
        &self.options
    }
}

impl Drop for ManagedSerialDevice {
    fn drop(&mut self) {
        let _ = self.outbound_handle.abort();
        let _ = self.inbound_handle.abort();
    }
}

#[derive(Default)]
pub struct SerialServer {
    managed: Arc<Mutex<HashMap<String, ManagedSerialDevice>>>,
}

fn api_raw_parity_2_sp(parity: i32) -> Option<serialport::Parity> {
    let mp = api::Parity::try_from(parity);
    mp.map(|p| match p {
        api::Parity::NoParity => serialport::Parity::None,
        api::Parity::Odd => serialport::Parity::Odd,
        api::Parity::Even => serialport::Parity::Even,
    })
    .ok()
}

fn api_raw_flow_2_sp(flow: i32) -> Option<serialport::FlowControl> {
    let mf = api::FlowControl::try_from(flow);
    mf.map(|f| match f {
        api::FlowControl::NoFlowControl => serialport::FlowControl::None,
        api::FlowControl::Software => serialport::FlowControl::Software,
        api::FlowControl::Hardware => serialport::FlowControl::Hardware,
    })
    .ok()
}

fn api_raw_data_bits_2_sp(data_bits: i32) -> Option<serialport::DataBits> {
    let mdb = api::DataBits::try_from(data_bits);
    mdb.map(|db| match db {
        api::DataBits::Five => serialport::DataBits::Five,
        api::DataBits::Six => serialport::DataBits::Six,
        api::DataBits::Seven => serialport::DataBits::Seven,
        api::DataBits::Eight => serialport::DataBits::Eight,
    })
    .ok()
}

fn api_raw_stop_bit_2_sp(stop_bits: i32) -> Option<serialport::StopBits> {
    let msb = api::StopBits::try_from(stop_bits);
    msb.map(|sb| match sb {
        api::StopBits::One => serialport::StopBits::One,
        api::StopBits::Two => serialport::StopBits::Two,
    })
    .ok()
}

// https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md
#[tonic::async_trait]
impl service::SerialService for SerialServer {
    type ReadStream = tonic::codec::Streaming<api::Bytes>;
    async fn list(
        &self,
        _req: tonic::Request<()>,
    ) -> Result<tonic::Response<api::ListResponse>, tonic::Status> {
        let mut response = api::ListResponse::default();
        match serialport::available_ports() {
            Ok(ports) => {
                for port in ports {
                    let managed = self.managed.lock().await;
                    let options = managed.get(&port.port_name).map(|s| s.options.clone());
                    let mut s = api::Serial::default();
                    s.device = port.port_name;
                    s.managed = options;
                    response.serials.push(s);
                }
                Ok(tonic::Response::new(response))
            }
            Err(e) => {
                error!("error listing serial ports: {}", e);
                return Err(tonic::Status::internal(e.description));
            }
        }
    }
    async fn open(
        &self,
        req: tonic::Request<api::OpenRequest>,
    ) -> Result<tonic::Response<api::Serial>, tonic::Status> {
        let req = req.into_inner();
        let options = req.options;
        if options.is_none() {
            return Err(tonic::Status::invalid_argument("options must be specified"));
        }
        let options = options.unwrap();
        let parity = api_raw_parity_2_sp(options.parity).unwrap_or(serialport::Parity::None);
        let stop = api_raw_stop_bit_2_sp(options.stop_bits).unwrap_or(serialport::StopBits::One);
        let flow = api_raw_flow_2_sp(options.flow_control).unwrap_or(serialport::FlowControl::None);
        let data = api_raw_data_bits_2_sp(options.data_bits).unwrap_or(serialport::DataBits::Eight);
        let prost_timeout = options.timeout.clone().unwrap_or_default();
        let timeout = std::time::Duration::from_nanos(prost_timeout.nanos as u64)
            + std::time::Duration::from_secs(prost_timeout.seconds as u64);
        let res = serialport::new(&req.device, options.baud)
            .data_bits(data)
            .parity(parity)
            .stop_bits(stop)
            .flow_control(flow)
            .timeout(timeout)
            .open_native_async();
        match res {
            Ok(port) => {
                // https://github.com/tokio-rs/tokio/blob/master/examples/echo-udp.rs
                // https://en.wikipedia.org/wiki/Registered_port
                // https://stackoverflow.com/questions/67443847/how-to-generate-random-numbers-in-async-rust
                let udp_port = {
                    let mut rng = rand::thread_rng();
                    rng.gen_range(49152..65535)
                };
                // TODO: bind to a random port and listen for incoming data
                // bind the udp to outbound rx and inbound tx
                let udp_addr = format!("0.0.0.0:{}", udp_port);
                let socket = UdpSocket::bind(udp_addr)
                    .await
                    .map(|s| Arc::new(Mutex::new(s)))
                    .map_err(|e| Arc::new(e)); // need an Arc to make error Clone
                let socket_ = socket.clone();
                let mut managed_options = api::ManagedOptions::default();
                // https://github.com/tokio-rs/tokio/discussions/3891
                let (tx, rx) = flume::bounded::<BufferType>(8);
                let out_tx = Arc::new(tx);
                let out_rx = Arc::new(rx);
                managed_options.options = Some(options.clone());
                managed_options.udp_port = if socket.is_ok() { udp_port } else { -1 };
                let pinned_port = Arc::pin(Mutex::new(SyncSerialStream(port)));
                let pinned_port_ = pinned_port.clone();
                let out_tx_ = out_tx.clone();
                let out_rx_ = out_rx.clone();
                let out_handle = tokio::spawn(async move {
                    let mut buf = [0u8; 512];
                    // https://v0-1--tokio.netlify.app/docs/io/async_read_write/
                    loop {
                        let r = pinned_port_.lock().await.read(&mut buf).await;
                        match r {
                            Ok(n) => {
                                let mut v = std::vec::Vec::with_capacity(n);
                                v.extend_from_slice(&buf[0..n]);
                                if let Some(mut c) = out_rx_.capacity() {
                                    while c >= out_tx_.len() {
                                        debug!("full channel for {} >= {}", out_tx_.len(), c);
                                        let _ = out_rx_.recv_async().await;
                                        c = out_rx_.capacity().unwrap_or(0);
                                    }
                                } else {
                                    debug!("unbounded channel");
                                }
                                match out_tx_.send(v) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        error!("error sending to channel: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("error reading from serial port: {}", e);
                            }
                        }
                    }
                });
                let (in_tx, in_rx) = flume::bounded::<BufferType>(8);
                let in_tx = Arc::new(in_tx);
                let in_rx = Arc::new(in_rx);
                let in_rx_ = in_rx.clone();
                let pinned_port_ = pinned_port.clone();
                let in_handle = tokio::spawn(async move {
                    for data in in_rx_.iter() {
                        match pinned_port_.lock().await.write_all(&data).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("error writing to serial port: {}", e);
                            }
                        }
                    }
                });
                // https://github.com/tokio-rs/tokio/discussions/3891
                // https://hackernoon.com/pin-safety-understanding-pinning-in-rust-futures
                // https://v0-1--tokio.netlify.app/docs/internals/net/
                let managed_dev = ManagedSerialDevice {
                    port: pinned_port,
                    port_name: req.device.clone(),
                    options: managed_options.clone(),
                    udp: None,
                    outbound_handle: out_handle,
                    inbound_handle: in_handle,
                    outbound: Channel {
                        rx: out_rx,
                        tx: out_tx,
                    },
                    inbound: Channel {
                        rx: in_rx,
                        tx: in_tx,
                    },
                };
                let mut response = api::Serial::default();
                // https://github.com/hyperium/tonic/discussions/1094
                let mut managed = self.managed.lock().await;
                managed.insert(req.device.clone(), managed_dev);
                response.device = req.device;
                response.managed = Some(managed_options);
                Ok(tonic::Response::new(response))
            }
            Err(e) => Err(tonic::Status::internal(e.description)),
        }
    }
    async fn close(
        &self,
        req: tonic::Request<api::CloseRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
    async fn read(
        &self,
        req: tonic::Request<api::ReadRequest>,
    ) -> Result<tonic::Response<Self::ReadStream>, tonic::Status> {
        todo!()
    }
    async fn write(
        &self,
        req: tonic::Request<api::WriteRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
