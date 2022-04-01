use serenity::model::channel::Message;
use crate::base::memory::MemoryChange;
use crate::base::command::Command;
use crate::services::*;

pub const SERVICES : [Service; 1]= [liers_dice::LIERS_DICE_SERVICE];

pub struct Service {
    pub identifier : &'static str,
    pub handler : fn(Command) -> (Message, MemoryChange),
}
