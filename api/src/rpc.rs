use proto::{splitter_server, *};
use tonic::{Request, Response, Status};

pub mod proto {
    tonic::include_proto!("splitter");
}

pub struct SplitterRpc {}

impl SplitterRpc {
    pub fn new() -> splitter_server::SplitterServer<SplitterRpc> {
        splitter_server::SplitterServer::new(Self {})
    }
}

#[tonic::async_trait]
impl splitter_server::Splitter for SplitterRpc {
    async fn create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<CreateAccountResponse>, Status> {
        dbg!(request);
        return Ok(Response::new(CreateAccountResponse {}));
    }
}
