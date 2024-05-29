mod rpc;
mod database;

use rpc::*;
use crate::database::Db;

use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use tonic::transport::Server;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool_options = SqliteConnectOptions::new()
        .filename("./sqlite.db")
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(pool_options).await?;
    sqlx::migrate!().run(&pool).await?;

    let service = SplitterRpc::new(Db::new(pool));

    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Serving on {}", addr);
    Server::builder()
        // GrpcWeb is over http1 so we must enable it.
        .accept_http1(true)
        .add_service(tonic_web::enable(service))
        .serve(addr)
        .await?;

    Ok(())
}
