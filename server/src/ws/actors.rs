use std::net::SocketAddr;

use futures::StreamExt;
use tiny_tokio_actor::{Actor, ActorContext, ActorSystem, Handler, Message, SystemEvent};
use tokio::{sync::mpsc, task};
use async_trait::async_trait;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::WebSocket;

use super::messages::ServerEvent;


#[derive(Clone)]
struct EchoActor {
    sender: mpsc::UnboundedSender<warp::ws::Message>
}

impl EchoActor {
    pub fn new(sender: mpsc::UnboundedSender<warp::ws::Message>) -> Self {
        EchoActor {
            sender
        }
    }
}

impl Actor<ServerEvent> for EchoActor {}

#[derive(Clone, Debug)]
struct EchoRequest(warp::ws::Message);

impl Message for EchoRequest {
    type Response = ();
}

#[async_trait]
impl Handler<ServerEvent, EchoRequest> for EchoActor {
    async fn handle(&mut self, msg: EchoRequest, ctx: &mut ActorContext<ServerEvent>) {
        ::log::debug!("actor {} on system {} received message! {:?}", &ctx.path, ctx.system.name(), &msg);
        self.sender.send(msg.0).unwrap()
    }
}


// Starts a new echo actor on our actor system
pub async fn start_echo(system: ActorSystem<ServerEvent>, remote: Option<SocketAddr>, websocket: WebSocket) {

    // Split out the websocket into incoming and outgoing
    let (ws_out, mut ws_in) = websocket.split();

    // Create an unbounded channel where the actor can send its responses to ws_out
    let (sender, receiver) = mpsc::unbounded_channel();
    let receiver = UnboundedReceiverStream::new(receiver);
    task::spawn(receiver.map(Ok).forward(ws_out));

    // Create a new echo actor with the newly created sender
    let actor = EchoActor::new(sender);
    // Use the websocket client address to generate a unique actor name
    let addr = remote
        .map(|addr| addr.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string() );
    let actor_name = format!("echo-actor-{}", &addr);
    // Launch the actor on our actor system
    let mut actor_ref = system.create_actor(&actor_name, actor).await.unwrap();

    // Loop over all websocket messages received over ws_in
    while let Some(result) = ws_in.next().await {
        // If no error, we tell the websocket message to the echo actor, otherwise break the loop
        match result {
            Ok(msg) => actor_ref.tell(EchoRequest(msg)).unwrap(),
            Err(error) => {
                ::log::error!("error processing ws message from {}: {:?}", &addr, error);
                break;
            }
        };
    }

    // The loop has been broken, kill the echo actor
    system.stop_actor(actor_ref.get_path()).await;
}
