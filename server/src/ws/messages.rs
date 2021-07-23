use bomber_shared::messages;
use tiny_tokio_actor::{Actor, ActorContext, ActorPath, Handler, Message, SystemEvent};
use tokio::sync::mpsc::{self, UnboundedSender};
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct ServerEvent(String);

// Mark the struct as a system event message.
impl SystemEvent for ServerEvent {}



#[derive(Clone, Debug)]
pub struct Transmission(pub Uuid, pub messages::message::MessagesTx);


impl Message for Transmission {
    type Response = Option<ActorPath>;
}

#[derive(Clone, Debug)]
pub struct Connect(pub Uuid, pub UnboundedSender<warp::ws::Message>);

impl Message for Connect {
    type Response = ();
}
