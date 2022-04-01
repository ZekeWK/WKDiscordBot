use crate::base::{
    error::CommandError,
    memory::MemoryChange,
    service::Service,
    command::Command,
};

use serenity::model::channel::Message;

pub const LIERS_DICE_SERVICE : Service = Service{ identifier : "liers_dice", handler : liers_dice_handler};

fn liers_dice_handler(command : Command) -> (Message, MemoryChange) {
    todo!()
}