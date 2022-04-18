use serenity::{builder::CreateMessage, model::{id::{ChannelId, UserId}}};
use crate::{base::{command::Command, memory::Data}, services::*};

pub const SERVICES : [Service; 6]= [dice::DICE_SERVICE, liars_dice::LIARS_DICE_SERVICE, info::ping::PING_SERVICE, info::help::HELP_SERVICE, info::count::COUNT_SERVICE, info::info::INFO_SERVICE];

#[derive(Clone, Copy)]
pub struct Service {
    pub identifier : &'static str,
    pub handler : ServiceHandler,
}

pub type ServiceHandler = fn(Command, Option<Data>) -> Vec<(ToMessage, CreateMessage<'static>)>;

pub enum ToMessage {
    User(UserId),
    Channel(ChannelId),
}

pub mod service_prelude {
    pub use crate::base::{service::{Service, ServiceHandler, ToMessage::{self, *}}, command::{Command, ArgsIter}, memory::{Data, to_struct, attachment_from_struct, from_struct}};
    pub use serenity::{model::id::ChannelId, builder::CreateMessage};
}