mod rpc;
use rpc::*;

use surrealdb::{engine::local::SpeeDb, Surreal};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create database connection
    let db = Surreal::new::<SpeeDb>("./speedb").await?;
    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    let addr = "127.0.0.1:3000".parse().unwrap();
    let service = SplitterRpc::new();

    Server::builder()
        // GrpcWeb is over http1 so we must enable it.
        .accept_http1(true)
        .add_service(tonic_web::enable(service))
        .serve(addr)
        .await?;

    Ok(())
}
