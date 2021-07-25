
use futures::StreamExt;
use tiny_tokio_actor::ActorSystem;
use tokio::{
    sync::mpsc,
    task,
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::WebSocket;


mod game;
mod lobby;
mod ws;

use self::ws::WsConn;
use super::messages::ServerEvent;

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
