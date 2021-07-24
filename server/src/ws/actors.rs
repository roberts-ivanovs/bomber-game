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

use super::messages::{Connect, ServerEvent, Transmission};

#[derive(Clone)]
struct WsConn {
    connection_id: Uuid,
    hb: Instant,
    websocket: UnboundedSender<warp::ws::Message>,
    communication_actor: Option<ActorPath>,
}

impl WsConn {
    async fn new(
        websocket: UnboundedSender<warp::ws::Message>,
        system: &ActorSystem<ServerEvent>,
    ) -> ActorRef<ServerEvent, Self> {
        let connection_id = Uuid::new_v4();
        let hb = Instant::now();

        let conn = Self {
            connection_id,
            hb,
            websocket,
            communication_actor: None,
        };

        let lounge_actor_path = system
            .create_actor("lounge", conn)
            .await
            .expect("Could not create lounge actor!");
        lounge_actor_path
    }

    async fn listen(
        ws_rx: &mut SplitStream<WebSocket>,
        actor_path: &mut ActorRef<ServerEvent, Self>,
    ) {
        // Loop over all websocket messages received over ws_rx
        while let Some(result) = ws_rx.next().await {
            // If no error, we tell the websocket message to the echo actor, otherwise break the loop
            match result {
                // Only accept binary messages
                Ok(msg) if msg.is_binary() => {
                    let deserialized_msg: message::MessagesTx =
                        nanoserde::DeBin::deserialize_bin(&msg.as_bytes())
                            .expect("Cant parse message");
                    actor_path
                        .tell(Transmission(deserialized_msg))
                        .expect("Could not `ask` the actor...");
                }
                _ => {
                    ::log::error!("error processing ws message!");
                }
            };
        }
    }
}

#[async_trait]
impl Actor<ServerEvent> for WsConn {}

#[async_trait]
impl Handler<ServerEvent, Transmission> for WsConn {
    async fn handle(
        &mut self,
        msg: Transmission,
        ctx: &mut ActorContext<ServerEvent>,
    ) {
        let return_actor = match msg.0 {
            message::MessagesTx::CreateLobby => {
                let lobby_id = Uuid::new_v4();
                let lobby = LobbyActor::new(lobby_id);
                let lobby_actor = ctx
                    .system
                    .create_actor(&lobby_id.to_string(), lobby)
                    .await
                    .expect("Impossible Uuid clash?");

                let message = warp::ws::Message::binary(message::MessagesRx::NewLobbyId {
                    lobby_id: lobby_id.as_bytes().clone(),
                });

                // if `send` is an error, then the connection is dropped.
                match self.websocket.send(message) {
                    Ok(_) => todo!(),
                    Err(e) =>  {
                        // TODO Close off the actor connection here
                        log::error!("Error sending message:{:?}", e);
                    },
                };

                Some(lobby_actor.get_path().to_owned())
            }
            message::MessagesTx::JoinLobby { username, lobby_id } => {
                // ctx.system.get_actor()
                // TODO Left off
                None
            }

            // Impossible...
            message::MessagesTx::PlayerState(_) => None,
            message::MessagesTx::Disconnect => None,
        };
        self.communication_actor = return_actor;
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
pub async fn handle_connection(system: ActorSystem<ServerEvent>, websocket: WebSocket) {
    // Split out the websocket into incoming and outgoing
    let (user_ws_tx, mut user_ws_rx) = websocket.split();

    // Create an unbounded channel where the actor can send its responses to user_ws_tx
    let (sender, receiver) = mpsc::unbounded_channel();
    let receiver = UnboundedReceiverStream::new(receiver);
    task::spawn(receiver.map(Ok).forward(user_ws_tx));

    let mut ws_conn = WsConn::new(sender, &system).await;
    WsConn::listen(&mut user_ws_rx, &mut ws_conn).await;
}
