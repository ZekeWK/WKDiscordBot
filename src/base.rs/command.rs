//TODO Will later allow for state like behaviour within threads. That is what the memory module will be for.

use crate::base::{error::CommandError, service::{Service, SERVICES, ServiceHandler}};
use serenity::model::{id::{ChannelId, UserId}, channel::Message};

pub const PREFIX : &str = "Argon,";
pub const SEPARATOR : &str = " ";

pub type ArgsIter<'a> = std::iter::Filter<std::str::Split<'a, &'static str>, fn(&&'a str) -> bool>;

pub struct Command<'a> {
    pub author : UserId,
    pub channel : ChannelId,
    pub args : ArgsIter<'a>,
}

impl <'a> Command<'a> {
    fn new(author : UserId, channel : ChannelId, args : ArgsIter<'a>) -> Command<'a> {
        Command{author : author, channel : channel, args : args}
    }
}

pub fn command_parse<'a>(message : &'a Message) -> Result<(ServiceHandler, Command), CommandError> {
    let author = message.author.id;
    let channel = message.channel_id;

    let mut args : ArgsIter = message.content.split(SEPARATOR).filter(|x| *x != SEPARATOR);
    
    if args.next() != Some(PREFIX) {
        return Err(CommandError::NotBot);
    }

    let service_identifier = match args.next() {
        Some(val) => val,
        None => return Err(CommandError::MissingService),
    };

    for service in SERVICES {
        if service.identifier != service_identifier {continue;}
        return Ok((service.handler, Command::new(author, channel, args)));
    }

    return Err(CommandError::NotService);
}
