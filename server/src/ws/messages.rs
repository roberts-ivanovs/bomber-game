use bomber_shared::messages::{self, message::Username};
use tiny_tokio_actor::{Actor, ActorContext, ActorPath, Handler, Message, SystemEvent};
use tokio::sync::mpsc::{self, UnboundedSender};
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct ServerEvent(String);

// Mark the struct as a system event message.
impl SystemEvent for ServerEvent {}



#[derive(Clone, Debug)]
pub struct Transmission(pub Vec<u8>);


impl Message for Transmission {
    type Response = ();
}

#[derive(Clone, Debug)]
pub struct Connect(pub Uuid, pub UnboundedSender<warp::ws::Message>, pub Username);

impl Message for Connect {
    type Response = ();
}


#[derive(Clone, Debug)]
pub struct Disconnect(pub Uuid);

impl Message for Disconnect {
    type Response = ();
}
