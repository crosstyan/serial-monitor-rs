use crate::serial::api::out::serial_service_server as service;
use crate::serial::api::out as api;

#[derive(Debug, Default)]
pub struct SerialServer;

#[tonic::async_trait]
impl service::SerialService for SerialServer {
  type ReadStream = tonic::codec::Streaming<api::Bytes>;
  async fn list(
    &self, _request: tonic::Request<()>
  ) -> Result<tonic::Response<api::ListResponse>, tonic::Status> {
    todo!()
  }
  async fn open(
    &self, _request: tonic::Request<api::OpenRequest>
  ) -> Result<tonic::Response<api::Serial>, tonic::Status> {
    todo!()
  }
  async fn close(
    &self, _request: tonic::Request<api::CloseRequest>
  ) -> Result<tonic::Response<()>, tonic::Status> {
    todo!()
  }
  async fn read(
    &self, _request: tonic::Request<api::ReadRequest>
  ) -> Result<tonic::Response<Self::ReadStream>, tonic::Status> {
    todo!()
  }
  async fn write(
    &self, _request: tonic::Request<api::WriteRequest>
  ) -> Result<tonic::Response<()>, tonic::Status> {
    todo!()
  }
}
