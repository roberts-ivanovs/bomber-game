use std::{collections::HashMap, fmt::format, net::SocketAddr};

use async_trait::async_trait;
use bomber_shared::messages::message;
use futures::StreamExt;
use tiny_tokio_actor::{
    Actor, ActorContext, ActorPath, ActorSystem, Handler, Message, SystemEvent,
};
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task,
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::WebSocket;

use super::messages::{Connect, ServerEvent, Transmission};

#[derive(Clone)]
pub struct LoungeActor {
    pub users: HashMap<Uuid, UnboundedSender<warp::ws::Message>>,
}

impl LoungeActor {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }
}
impl Actor<ServerEvent> for LoungeActor {}

#[async_trait]
impl Handler<ServerEvent, Connect> for LoungeActor {
    async fn handle(&mut self, msg: Connect, ctx: &mut ActorContext<ServerEvent>) {
        self.users.insert(msg.0, msg.1); // NOTE: unhandled Option here
    }
}

#[async_trait]
impl Handler<ServerEvent, Transmission> for LoungeActor {
    async fn handle(
        &mut self,
        msg: Transmission,
        ctx: &mut ActorContext<ServerEvent>,
    ) -> Option<ActorPath> {
        let return_actor = match msg.1 {
            message::MessagesTx::CreateLobby => {
                let lobby_id = Uuid::new_v4();
                let lobby = LobbyActor::new(lobby_id);
                let lobby_actor = ctx
                    .system
                    .create_actor(&lobby_id.to_string(), lobby)
                    .await
                    .expect("Impossible Uuid clash?");

                self.users.get(&msg.0).map(|sender| {
                    let message = warp::ws::Message::binary(message::MessagesRx::NewLobbyId {
                        lobby_id: lobby_id.as_bytes().clone(),
                    });
                    // TODO if `send` is an error, then the connection is dropped.
                    sender.send(message);
                });
                Some(lobby_actor.get_path().to_owned())
            }
            message::MessagesTx::JoinLobby { username, lobby_id } => {
                // ctx.system.get_actor()
                // TODO Left off
                None
            },

            // Impossible...
            message::MessagesTx::PlayerState(_) => None,
            message::MessagesTx::Disconnect => None,
        };
        return return_actor;
    }
}

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

// Starts a new echo actor on our actor system
pub async fn handle_connection(
    system: ActorSystem<ServerEvent>,
    lounge_path: ActorPath,
    websocket: WebSocket,
) {
    // Split out the websocket into incoming and outgoing
    let (user_ws_tx, mut user_ws_rx) = websocket.split();

    // Create an unbounded channel where the actor can send its responses to user_ws_tx
    let (sender, receiver) = mpsc::unbounded_channel();
    let receiver = UnboundedReceiverStream::new(receiver);
    task::spawn(receiver.map(Ok).forward(user_ws_tx));

    let mut lounge = system
        .get_actor::<LoungeActor>(&lounge_path)
        .await
        .expect("Could not get the lounge actor!");

    let connection_id = Uuid::new_v4();
    lounge
        .tell(Connect(connection_id, sender))
        .expect("Could not `tell` the lounge about a new connection");

    // Loop over all websocket messages received over user_ws_rx
    while let Some(result) = user_ws_rx.next().await {
        // If no error, we tell the websocket message to the echo actor, otherwise break the loop
        match result {
            // Only accept binary messages
            Ok(msg) if msg.is_binary() => {
                let deserialized_msg: message::MessagesTx =
                    nanoserde::DeBin::deserialize_bin(&msg.as_bytes()).expect("Cant parse message");

                let resp = lounge
                    .ask(Transmission(connection_id, deserialized_msg))
                    .await
                    .expect("Could not `ask` the actor...");
                if let Some(new_path) = resp {
                    lounge = system
                        .get_actor(&new_path)
                        .await
                        .expect("Could not find actor");
                }
            }
            _ => {
                ::log::error!("error processing ws message from {}", &connection_id);
                break;
            }
        };
    }
}
