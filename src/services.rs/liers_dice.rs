use crate::base::memory::Data;
use crate::base::{
    error::CommandError,
    service::Service,
    command::Command,
};

use serenity::model::id::ChannelId;
use serenity::builder::CreateMessage;

pub const LIERS_DICE_SERVICE : Service = Service{ identifier : "liers_dice", handler : liers_dice_handler};

fn liers_dice_handler(command : Command, data : Option<Data>) -> Vec<(ChannelId, CreateMessage<'static>)> {
    todo!()
}