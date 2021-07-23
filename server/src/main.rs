mod startup;
mod api;
mod how;
mod ws;
mod routes;

use std::net::SocketAddr;
use std::str::FromStr;

use warp::*;


#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info,tiny_tokio_actor=debug,websocket=debug");
    }
    env_logger::init();
    // ---------------------------------- //
    let path = std::path::Path::new(".env");
    dotenv::from_path(path).ok();

    let addr = std::env::var("HOST")
        .ok()
        .and_then(|string| SocketAddr::from_str(&string).ok())
        .unwrap_or_else(|| SocketAddr::from_str("127.0.0.1:9000").unwrap());
    // ---------------------------------- //
    // Run the server
    startup::run(addr).await.await;
}
