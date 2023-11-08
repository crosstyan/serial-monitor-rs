///
/// We don't need error type since gRPC already has it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Serial {
    /// *
    /// /dev/tty (Linux)
    /// /dev/cu  (Mac)
    /// COM      (Windows)
    #[prost(string, tag = "1")]
    pub device: ::prost::alloc::string::String,
    ///
    /// If the serial is managed by the server, we would see its baud rate and ports
    #[prost(message, optional, tag = "2")]
    pub managed: ::core::option::Option<ManagedOptions>,
}
/// <https://docs.rs/serialport/latest/serialport/struct.SerialPortBuilder.html>
/// <https://github.com/protocolbuffers/protobuf/blob/main/src/google/protobuf/duration.proto>
/// the most common should be 8N1 (i.e. 8 data bits, no parity, 1 stop bit)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenOptions {
    #[prost(uint32, tag = "1")]
    pub baud: u32,
    #[prost(enumeration = "DataBits", tag = "2")]
    pub data_bits: i32,
    #[prost(enumeration = "FlowControl", tag = "3")]
    pub flow_control: i32,
    #[prost(enumeration = "Parity", tag = "4")]
    pub parity: i32,
    #[prost(enumeration = "StopBits", tag = "5")]
    pub stop_bits: i32,
    /// Set the amount of time to wait to receive data before timing out
    #[prost(message, optional, tag = "6")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagedOptions {
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<OpenOptions>,
    /// *
    /// outside -> serial device (write/spit)
    /// outside <- serial device (read/slurp)
    #[prost(int32, tag = "2")]
    pub udp_port: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListResponse {
    #[prost(message, repeated, tag = "1")]
    pub serials: ::prost::alloc::vec::Vec<Serial>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenRequest {
    #[prost(string, tag = "1")]
    pub device: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub options: ::core::option::Option<OpenOptions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bytes {
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRequest {
    #[prost(string, tag = "1")]
    pub device: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteRequest {
    #[prost(string, tag = "1")]
    pub device: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseRequest {
    #[prost(string, tag = "1")]
    pub device: ::prost::alloc::string::String,
}
/// * <https://docs.rs/serialport/latest/serialport/enum.DataBits.html>
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataBits {
    /// most common
    Eight = 0,
    Five = 1,
    Six = 2,
    Seven = 3,
}
impl DataBits {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataBits::Eight => "Eight",
            DataBits::Five => "Five",
            DataBits::Six => "Six",
            DataBits::Seven => "Seven",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Eight" => Some(Self::Eight),
            "Five" => Some(Self::Five),
            "Six" => Some(Self::Six),
            "Seven" => Some(Self::Seven),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlowControl {
    NoFlowControl = 0,
    /// Flow control using XON/XOFF bytes
    Software = 1,
    /// Flow control using RTS/CTS signals
    Hardware = 2,
}
impl FlowControl {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FlowControl::NoFlowControl => "NoFlowControl",
            FlowControl::Software => "Software",
            FlowControl::Hardware => "Hardware",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NoFlowControl" => Some(Self::NoFlowControl),
            "Software" => Some(Self::Software),
            "Hardware" => Some(Self::Hardware),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Parity {
    NoParity = 0,
    Odd = 1,
    Even = 2,
}
impl Parity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Parity::NoParity => "NoParity",
            Parity::Odd => "Odd",
            Parity::Even => "Even",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NoParity" => Some(Self::NoParity),
            "Odd" => Some(Self::Odd),
            "Even" => Some(Self::Even),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StopBits {
    One = 0,
    Two = 1,
}
impl StopBits {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StopBits::One => "One",
            StopBits::Two => "Two",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "One" => Some(Self::One),
            "Two" => Some(Self::Two),
            _ => None,
        }
    }
}
/// Generated server implementations.
pub mod serial_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SerialServiceServer.
    #[async_trait]
    pub trait SerialService: Send + Sync + 'static {
        ///
        /// @brief List all available serial devices
        /// @return Returns a list of all serial ports on system
        /// @note It is not guaranteed that these ports exist or are available even if they're returned by this function, unless it's managed
        async fn list(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<super::ListResponse>, tonic::Status>;
        ///
        /// @brief Open a serial connection to a device
        /// @param OpenSerialRequest The request message containing the device name and baud rate
        /// @return Serial The opened serial
        async fn open(
            &self,
            request: tonic::Request<super::OpenRequest>,
        ) -> std::result::Result<tonic::Response<super::Serial>, tonic::Status>;
        ///
        /// @brief Close the serial
        /// @param device The device to close
        async fn close(
            &self,
            request: tonic::Request<super::CloseRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Server streaming response type for the Read method.
        type ReadStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::Bytes, tonic::Status>,
            >
            + Send
            + 'static;
        ///
        /// @brief Read data from a serial device
        /// @param ReadRequest The request message containing the device name
        /// @return stream Bytes A stream of Bytes messages containing the read data
        async fn read(
            &self,
            request: tonic::Request<super::ReadRequest>,
        ) -> std::result::Result<tonic::Response<Self::ReadStream>, tonic::Status>;
        ///
        /// @brief Write data to a serial device
        /// @param WriteRequest The request message containing the device name and the data to write
        /// @note If you have access to the UDP protocol, you should use it instead
        async fn write(
            &self,
            request: tonic::Request<super::WriteRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SerialServiceServer<T: SerialService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SerialService> SerialServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SerialServiceServer<T>
    where
        T: SerialService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/serial.api.SerialService/List" => {
                    #[allow(non_camel_case_types)]
                    struct ListSvc<T: SerialService>(pub Arc<T>);
                    impl<T: SerialService> tonic::server::UnaryService<()>
                    for ListSvc<T> {
                        type Response = super::ListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SerialService>::list(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serial.api.SerialService/Open" => {
                    #[allow(non_camel_case_types)]
                    struct OpenSvc<T: SerialService>(pub Arc<T>);
                    impl<
                        T: SerialService,
                    > tonic::server::UnaryService<super::OpenRequest> for OpenSvc<T> {
                        type Response = super::Serial;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpenRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SerialService>::open(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OpenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serial.api.SerialService/Close" => {
                    #[allow(non_camel_case_types)]
                    struct CloseSvc<T: SerialService>(pub Arc<T>);
                    impl<
                        T: SerialService,
                    > tonic::server::UnaryService<super::CloseRequest> for CloseSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CloseRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SerialService>::close(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CloseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serial.api.SerialService/Read" => {
                    #[allow(non_camel_case_types)]
                    struct ReadSvc<T: SerialService>(pub Arc<T>);
                    impl<
                        T: SerialService,
                    > tonic::server::ServerStreamingService<super::ReadRequest>
                    for ReadSvc<T> {
                        type Response = super::Bytes;
                        type ResponseStream = T::ReadStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SerialService>::read(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serial.api.SerialService/Write" => {
                    #[allow(non_camel_case_types)]
                    struct WriteSvc<T: SerialService>(pub Arc<T>);
                    impl<
                        T: SerialService,
                    > tonic::server::UnaryService<super::WriteRequest> for WriteSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WriteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SerialService>::write(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WriteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: SerialService> Clone for SerialServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: SerialService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SerialService> tonic::server::NamedService for SerialServiceServer<T> {
        const NAME: &'static str = "serial.api.SerialService";
    }
}
