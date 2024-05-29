use proto::{splitter_server, *};
use tonic::{Request, Response, Status};

use crate::database::{self as db, Db};

pub mod proto {
    tonic::include_proto!("splitter");
}

pub struct SplitterRpc {
    db: Db,
}

impl SplitterRpc {
    pub fn new(db: Db) -> splitter_server::SplitterServer<SplitterRpc> {
        splitter_server::SplitterServer::new(Self { db })
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

    async fn upsert_event(
        &self,
        request: Request<UpsertEventRequest>,
    ) -> Result<Response<UpsertEventResponse>, Status> {
        let inner = request
            .into_inner()
            .event
            .to_invalid_arg("Field event missing")?;

        self.db
            .insert_event(
                inner.amount.to_invalid_arg("Field inner missing")?,
                &inner.name.to_invalid_arg("Field name missing")?,
            )
            .await
            .map_err(|e| Status::from_error(e.into()))?;

        return Ok(Response::new(UpsertEventResponse {}));
    }

    async fn get_all_events(
        &self,
        _request: Request<GetAllEventsRequest>,
    ) -> Result<Response<GetAllEventsResponse>, Status> {
        let result = self
            .db
            .fetch_all_events()
            .await
            .map_err(|e| Status::from_error(e.into()))?;

        return Ok(Response::new(GetAllEventsResponse  { events: result }));
    }
}

pub trait ToInvalidArgument<T> {
    fn to_invalid_arg(self, msg: &str) -> Result<T, Status>;
}

impl<T> ToInvalidArgument<T> for Option<T> {
    fn to_invalid_arg(self, msg: &str) -> Result<T, Status> {
        self.ok_or_else(|| Status::invalid_argument(msg))
    }
}
