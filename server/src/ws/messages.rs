use tiny_tokio_actor::{Actor, ActorContext, Handler, Message, SystemEvent};
use tokio::sync::mpsc;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct ServerEvent(String);

// Mark the struct as a system event message.
impl SystemEvent for ServerEvent {}
