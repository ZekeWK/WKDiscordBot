//TODO Will later allow for state like behaviour within threads. That is what the memory module will be for.

use crate::base::{error::CommandError, service::{Service, SERVICES}};
use serenity::model::{id::ChannelId, user::User, channel::Message};

const PREFIX : &str = "Zeke,";
const SEPARATOR : &str = " ";

pub struct Command {
    pub author : User,
    pub channel : ChannelId,
    pub args : Vec<String>,
}

impl Command {
    fn new(author : User, channel : ChannelId, args : Vec<String>) -> Command {
        Command{author : author, channel : channel, args : args}
    }
}

pub fn command_parse(message : Message) -> Result<(Service, Command), CommandError> {
    let author = message.author;
    let channel = message.channel_id;

    let mut args = message.content.split(SEPARATOR).filter(|x| *x != SEPARATOR);
    
    if args.next() != Some(PREFIX) {
        return Err(CommandError::NotBot);
    }

    let service_identifier = match args.next() {
        Some(val) => val,
        None => return Err(CommandError::MissingService),
    };

    for service in SERVICES {
        if service.identifier != service_identifier {continue;}
        let args : Vec<String> = args.map(|x| x.to_string()).collect();

        return Ok((service, Command::new(author, channel, args)));
    }

    return Err(CommandError::NotService);
}
