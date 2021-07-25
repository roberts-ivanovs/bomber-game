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

use super::messages::{Connect, Disconnect, ServerEvent, Transmission};

#[derive(Clone)]
enum DeserializerType {
    MainMenu,
    Lobby(ActorPath),
    Game(ActorPath),
}

#[derive(Clone)]
struct WsConn {
    connection_id: Uuid,
    hb: Instant,
    websocket: UnboundedSender<warp::ws::Message>,
    comms: DeserializerType,
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
            comms: DeserializerType::MainMenu,
        };

        let ws_actor = system
            .create_actor(&connection_id.to_string(), conn)
            .await
            .expect("Could not create lounge actor!");
        ws_actor
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
                    actor_path
                        .tell(Transmission(msg.into_bytes()))
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
    async fn handle(&mut self, msg: Transmission, ctx: &mut ActorContext<ServerEvent>) {
        match &self.comms {
            DeserializerType::MainMenu => {
                let deserialized_msg: message::tx::MessagesMainMenu =
                    nanoserde::DeBin::deserialize_bin(&msg.0).expect("Cant parse message");

                let return_actor = match deserialized_msg {
                    message::tx::MessagesMainMenu::JoinLobby { username, lobby_id } => {
                        let lobby_id = Uuid::from_bytes(lobby_id);
                        let path: ActorPath = ActorPath::from("/user") / &lobby_id.to_string();
                        let lobby_actor = ctx
                            .system
                            .get_actor::<LobbyActor>(&path)
                            .await
                            .map(|mut lob| {
                                lob.tell(Connect(
                                    self.connection_id,
                                    self.websocket.clone(),
                                    username,
                                ))
                                .map(|_| {
                                    let message = warp::ws::Message::binary(
                                        message::rx::MessagesMainMenu::SuccessfulJoin,
                                    );
                                    // if `send` is an error, then the connection is dropped.
                                    self.websocket.send(message).unwrap();
                                })
                                .unwrap();
                                lob
                            })
                            .unwrap();
                        DeserializerType::Lobby(lobby_actor.get_path().to_owned())
                    }
                    message::tx::MessagesMainMenu::CreateLobby => {
                        let lobby_id = Uuid::new_v4();
                        let lobby = LobbyActor::new(lobby_id);
                        let lobby_actor = ctx
                            .system
                            .create_actor(&lobby_id.to_string(), lobby)
                            .await
                            .expect("Impossible Uuid clash?");

                        let message =
                            warp::ws::Message::binary(message::rx::MessagesMainMenu::NewLobbyId {
                                lobby_id: lobby_id.as_bytes().clone(),
                            });

                        // if `send` is an error, then the connection is dropped.
                        match self.websocket.send(message) {
                            Ok(_) => {
                                todo!()
                            }
                            Err(e) => {
                                // TODO Close off the actor connection here
                                log::error!("Error sending message:{:?}", e);
                            }
                        };
                        DeserializerType::Lobby(lobby_actor.get_path().to_owned())
                    }
                };
                self.comms = return_actor;
            }
            DeserializerType::Lobby(lobby_path) => {
                let deserialized_msg: message::tx::MessagesLobby =
                    nanoserde::DeBin::deserialize_bin(&msg.0).expect("Cant parse message");
                let mut lobby = ctx.system
                        .get_actor::<LobbyActor>(&lobby_path)
                        .await
                        .unwrap();
                let return_actor = match deserialized_msg {
                    message::tx::MessagesLobby::Disconnect => {
                        // Remove player from current lobby
                        lobby.tell(Disconnect(self.connection_id)).unwrap();
                        DeserializerType::MainMenu
                    }
                    message::tx::MessagesLobby::StartGame => {
                        // Create a new game actor, transfer all players to there.
                        todo!()
                        // DeserializerType::Game()
                    }
                };
                self.comms = return_actor;
            }
            DeserializerType::Game(_) => todo!(),
        }
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
