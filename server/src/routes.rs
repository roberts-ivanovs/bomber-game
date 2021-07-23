use std::convert::Infallible;

use warp::{hyper::StatusCode, Filter};

use crate::ws;

/// Define routes for the base application
pub async fn api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    (warp::path("health_check")
        .and(warp::get())
        .and_then(health_check))
    .or(ws::routes().await)
}

/// Simple health check handler to make sure that the server is indeed up and
/// running and is able to handle requests.
async fn health_check() -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}
