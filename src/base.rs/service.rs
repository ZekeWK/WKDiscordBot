use serenity::{builder::CreateMessage, model::id::ChannelId};
use crate::{base::command::Command, services::*};

pub const SERVICES : [Service; 4]= [dice::DICE_SERVICE, liers_dice::LIERS_DICE_SERVICE, info::ping::PING_SERVICE, info::help::HELP_SERVICE];

#[derive(Clone, Copy)]
pub struct Service {
    pub identifier : &'static str,
    pub handler : ServiceHandler,
}

pub type ServiceHandler = fn(Command) -> Vec<(ChannelId, CreateMessage<'static>)>;