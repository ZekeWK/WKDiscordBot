use crate::base::{service::Service, command::Command};
use serenity::{model::id::ChannelId, builder::CreateMessage};

pub const DICE_SERVICE : Service = Service{identifier : "dice", handler : dice_handler};

fn dice_handler(command : Command) -> (ChannelId, CreateMessage<'static>) {
    
    

    
    
    
    let message = todo!();
    
    return (command.channel, message);
}