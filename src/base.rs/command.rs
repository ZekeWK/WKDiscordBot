use crate::base::{error::CommandError, service::{SERVICES, Service}, secret::USER_MENTION};
use serenity::model::{id::{ChannelId, UserId, MessageId}, channel::{Message, MessageReference}};

pub const PREFIX : &str = USER_MENTION;
pub const SEPARATOR : &str = " ";

pub type ArgsIter<'a> = std::iter::Filter<std::str::Split<'a, &'static str>, fn(&&'a str) -> bool>;

#[derive(Debug, Clone)]
pub struct Command<'a> {
    pub author : UserId,
    pub channel : ChannelId,
    pub msg : MessageId,
    pub args : ArgsIter<'a>,
}

impl <'a> Command<'a> {
    fn new(author : UserId, channel : ChannelId, msg : MessageId, args : ArgsIter<'a>) -> Command<'a> {
        Command{author : author, channel : channel, msg : msg, args : args}
    }

    pub fn message_reference(&self) -> MessageReference {
        MessageReference::from((self.channel, self.msg))

    }
}

pub fn command_parse_prefix<'a>(msg : &'a Message) -> Result<(Service, Command), CommandError> {
    let mut args : ArgsIter = msg.content.split(SEPARATOR).filter(|x| *x != "");
    if args.next() != Some(PREFIX) {
        return Err(CommandError::NotBot);
    }
    
    let author = msg.author.id;
    let channel = msg.channel_id;
    let msg_id = msg.id;
    
    let service_identifier = match args.next() {
        Some(val) => val,
        None => return Err(CommandError::MissingService),
    };
    
    
    if let Some(service) = get_service(service_identifier) {
        return Ok((service, Command::new(author, channel, msg_id, args)));
    }

    return Err(CommandError::NotService);
}

pub fn command_parse<'a>(msg : &'a Message) -> Command<'a> {
    let author = msg.author.id;
    let channel = msg.channel_id;
    let msg_id = msg.id;

    let args : ArgsIter = msg.content.split(SEPARATOR).filter(|x| *x != "");

    Command::new(author, channel, msg_id, args)
}

pub fn get_service(identifier : &str) -> Option<Service> {
    for service in SERVICES {
        if service.identifier != identifier {continue;}
        return Some(service);
    }

    None
}