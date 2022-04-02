use crate::base::{service::Service, command::Command};
use serenity::{model::id::ChannelId, builder::CreateMessage};

pub const DICE_SERVICE : Service = Service{identifier : "dice", handler : dice_handler};

fn dice_handler(command : Command) -> (ChannelId, CreateMessage<'static>) {
    //1d6 + 3
    let mut dice : Vec<(u32, u32)> = Vec::new();

    let mut i = 0;
    
    
    
    
    let message = todo!();
    
    return (command.channel, message);
}