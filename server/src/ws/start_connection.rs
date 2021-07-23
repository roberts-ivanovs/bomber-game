use std::net::SocketAddr;

use tiny_tokio_actor::{ActorSystem, EventBus};
use warp::Filter;

use super::{
    actors::{handle_connection, LoungeActor},
    messages::ServerEvent,
};

pub async fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Create the event bus and actor system
    let bus = EventBus::<ServerEvent>::new(10_000);
    let system = ActorSystem::new("test", bus);
    let lounge_actor_path = system
        .create_actor("lounge", LoungeActor::new())
        .await
        .expect("Could not create lounge actor!");
    // Create the warp WebSocket route
    warp::path("game")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::any().map(move || system.clone()))
        .and(warp::any().map(move || lounge_actor_path.get_path().clone()))
        .and(warp::ws())
        .map(
            |system, lounge_path, ws: warp::ws::Ws| {
                ws.on_upgrade(move |websocket| handle_connection(system, lounge_path, websocket))
            },
        )
}
