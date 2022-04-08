use serenity::{builder::CreateMessage, model::id::ChannelId};
use crate::{base::{command::Command, memory::Data}, services::*};

pub const SERVICES : [Service; 5]= [dice::DICE_SERVICE, liers_dice::LIERS_DICE_SERVICE, info::ping::PING_SERVICE, info::help::HELP_SERVICE, info::count::COUNT_SERVICE];

#[derive(Clone, Copy)]
pub struct Service {
    pub identifier : &'static str,
    pub handler : ServiceHandler,
}

pub type ServiceHandler = fn(Command, Option<Data>) -> Vec<(ChannelId, CreateMessage<'static>)>;

pub mod service_prelude {
    pub use crate::base::{service::Service, command::Command, memory::{Data, to_struct, create_attachment, from_struct}};
    pub use serenity::{model::id::ChannelId, builder::CreateMessage};
}