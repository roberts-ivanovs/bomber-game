
use std::net::SocketAddr;

use futures::Future;
use warp::Filter;

use crate::routes;

/// Start up the application based on pre-defined variables
pub async fn run(addr: SocketAddr) -> impl Future<Output = ()> {
    let api = routes::api().await;

    // Create the warp routes (websocket only in this case, with warp logging added)
    let routes = api.with(warp::log("ws"));

    // Start the server
    let server = warp::serve(routes).run(addr);
    return server;
}
