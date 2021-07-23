use std::net::SocketAddr;

use tiny_tokio_actor::{ActorSystem, EventBus};
use warp::Filter;

use super::{actors::start_echo, messages::ServerEvent};

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    // Create the event bus and actor system
    let bus = EventBus::<ServerEvent>::new(10_000);
    let system = ActorSystem::new("test", bus);

    // Create the warp WebSocket route
    warp::path("game")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::any().map(move || system.clone()))
        .and(warp::addr::remote())
        .and(warp::ws())
        .map(|system: ActorSystem<ServerEvent>, remote: Option<SocketAddr>, ws: warp::ws::Ws| {
            ws.on_upgrade(move |websocket| start_echo(system, remote, websocket) )
        })
}
