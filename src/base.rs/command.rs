use crate::base::{error::CommandError, service::{Service, SERVICES}};
use serenity::model::{id::ChannelId, user::User, channel::{Message, self}};

const PREFIX : &str = "Zeke";
const SEPARATOR : &str = " ";

pub struct Command {
    author : User,
    channel : ChannelId,
    service : Service,
    args : Vec<String>,
}

impl Command {
    fn new(author : User, channel : ChannelId, service : Service, args : Vec<String>) -> Command {
        Command{author : author, channel : channel, service : service, args : args}
    }
}

pub fn command_parse(message : Message) -> Result<Command, CommandError> {
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

        return Ok(Command::new(author, channel, service, args));
    }

    return Err(CommandError::NotService);
}
