use crate::db::{connect, create_tables};
use crate::grpc::start_server as start_grpc_server;
use crate::routes::create_routes;
use tokio::join;
use tokio::net::TcpListener;

mod db;
mod grpc;
mod models;
mod routes;
mod schemas;
mod services;
mod pages;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    let db = connect().await;

    create_tables(&db).await;
    db.close().await.unwrap();

    let api = create_routes().await;

    let app = pages::create_pages_router()
    .await.merge(api);

    // Create a TcpListener to accept connections
    let listener = TcpListener::bind("0.0.0.0:3500").await.unwrap();

    println!(
        "HTTP server will start at {}",
        listener.local_addr().unwrap()
    );

    let grpc_server = start_grpc_server();

    // Start Axum server
    let http_server = axum::serve(listener, app.into_make_service());

    let (grpc_result, http_result) = join!(grpc_server, http_server);

    grpc_result.unwrap();
    http_result.unwrap();
}
