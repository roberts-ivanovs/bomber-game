use std::net::SocketAddr;

use tiny_tokio_actor::{ActorSystem, EventBus};
use warp::Filter;

use super::{
    actors::handle_connection,
    messages::ServerEvent,
};

pub async fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Create the event bus and actor system
    let bus = EventBus::<ServerEvent>::new(10_000);
    let system = ActorSystem::new("test", bus);
    // Create the warp WebSocket route
    warp::path("game")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::any().map(move || system.clone()))
        .and(warp::ws())
        .map(
            |system, ws: warp::ws::Ws| {
                ws.on_upgrade(move |websocket| handle_connection(system, websocket))
            },
        )
}
