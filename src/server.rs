use crate::serial::api::out as api;
use crate::serial::api::out::serial_service_server as service;
use tokio::sync::Mutex;
use serialport::SerialPort;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tracing::{debug, error, info, instrument, trace, warn};

pub struct ManagedSerialDevice {
    port: Box<dyn SerialPort>,
    port_name: String,
    options: api::ManagedOptions,
    udp: Option<UdpSocket>,
}

#[derive(Default)]
pub struct SerialServer {
    // path -> ManagedSerialDevice (which also contains the path)
    managed: Mutex<Arc<HashMap<String, ManagedSerialDevice>>>,
}

/// this server gives no shit about
/// multiple client using the same serial port
/// 
/// clients themselves should handle that, or
/// a third-party program could manage this
unsafe impl Send for SerialServer {}
unsafe impl Sync for SerialServer {}

fn api_raw_parity_2_sp(parity:i32) -> Option<serialport::Parity>{
    let mp = api::Parity::try_from(parity);
    mp.map(|p| match p {
            api::Parity::NoParity => serialport::Parity::None,
            api::Parity::Odd => serialport::Parity::Odd,
            api::Parity::Even => serialport::Parity::Even,
        }).ok()
}

fn api_raw_flow_2_sp(flow:i32) -> Option<serialport::FlowControl>{
    let mf = api::FlowControl::try_from(flow);
    mf.map(|f| match f {
            api::FlowControl::NoFlowControl => serialport::FlowControl::None,
            api::FlowControl::Software => serialport::FlowControl::Software,
            api::FlowControl::Hardware => serialport::FlowControl::Hardware,
        }).ok()
}

fn api_raw_data_bits_2_sp(data_bits:i32) -> Option<serialport::DataBits>{
    let mdb = api::DataBits::try_from(data_bits);
    mdb.map(|db| match db {
            api::DataBits::Five => serialport::DataBits::Five,
            api::DataBits::Six => serialport::DataBits::Six,
            api::DataBits::Seven => serialport::DataBits::Seven,
            api::DataBits::Eight => serialport::DataBits::Eight,
        }).ok()
}

fn api_raw_stop_bit_2_sp(stop_bits:i32) -> Option<serialport::StopBits>{
    let msb = api::StopBits::try_from(stop_bits);
    msb.map(|sb| match sb {
            api::StopBits::One => serialport::StopBits::One,
            api::StopBits::Two => serialport::StopBits::Two,
        }).ok()
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
                    let options = managed.get(&port.port_name).map(|s| s.options);
                    let mut s = api::Serial::default();
                    s.device = port.port_name;
                    s.managed = options;
                    response.serials.push(s);
                }
                Ok(tonic::Response::new(response))
            },
            Err(e) => {
                error!("error listing serial ports: {}", e);
                return Err(tonic::Status::internal(e.description))
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
        let prost_timeout = options.timeout.unwrap_or_default();
        let timeout = std::time::Duration::from_nanos(prost_timeout.nanos as u64)
            + std::time::Duration::from_secs(prost_timeout.seconds as u64);
        let res = serialport::new(&req.device, options.baud)
            .data_bits(data)
            .parity(parity)
            .stop_bits(stop)
            .flow_control(flow)
            .timeout(timeout)
            .open();
        match res {
            Ok(port) => {
                // https://github.com/tokio-rs/tokio/blob/master/examples/echo-udp.rs
                let udp_port = req.udp_port;
                if udp_port.is_negative() || udp_port == 0 {}
                let socket = UdpSocket::bind("").await;
                let mut managed_options = api::ManagedOptions::default();
                managed_options.options = Some(options);
                managed_options.udp_port = udp_port;
                let cm = managed_options.clone();
                let managed_dev = ManagedSerialDevice {
                    port: port,
                    port_name: req.device,
                    options: managed_options,
                    udp: None,
                };
                let mut response = api::Serial::default();
                // https://github.com/hyperium/tonic/discussions/1094
                let managed = self.managed.lock().await;
                managed.insert(req.device, managed_dev);
                response.device = req.device;
                response.managed = Some(cm);
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
