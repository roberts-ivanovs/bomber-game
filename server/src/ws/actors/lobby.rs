use std::{collections::HashMap, fmt::format, net::SocketAddr};

use async_trait::async_trait;
use bomber_shared::messages::message;
use futures::{stream::SplitStream, StreamExt};
use tiny_tokio_actor::{
    Actor, ActorContext, ActorPath, ActorRef, ActorSystem, Handler, Message, SystemEvent,
};
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task,
    time::Instant,
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::WebSocket;

use super::super::messages::{Connect, Disconnect, ServerEvent, Transmission};

#[derive(Clone)]
pub struct LobbyActor {
    lobby_id: Uuid,
    users: HashMap<Uuid, UnboundedSender<warp::ws::Message>>,
}
impl Actor<ServerEvent> for LobbyActor {}

impl LobbyActor {
    pub fn new(lobby_id: Uuid) -> Self {
        Self {
            lobby_id,
            users: HashMap::new(),
        }
    }
}

#[async_trait]
impl Handler<ServerEvent, Connect> for LobbyActor {
    async fn handle(&mut self, msg: Connect, ctx: &mut ActorContext<ServerEvent>) {
        todo!()
    }
}

#[async_trait]
impl Handler<ServerEvent, Disconnect> for LobbyActor {
    async fn handle(&mut self, msg: Disconnect, ctx: &mut ActorContext<ServerEvent>) {
        todo!()
    }
}
