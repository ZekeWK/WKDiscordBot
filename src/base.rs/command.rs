/* All commands should be on the format : "PREFIX,COMMANDTYPE,ARGS..." where , denotes the SEPARATOR  */



use crate::standard::error::CommandError;
use serenity::model::{id::ChannelId, user::User, channel::{Message, self}};

const SEPARATOR : &str = " ";
const PREFIX : &str = "Zeke ";
const NO_OP_WORD : &str = "NoOP";
const HELP_WORD : &str = "help";
const PING_WORD : &str = "ping";
const LIERS_DICE_WORD : &str = "liers_dice";

struct Command <I> where I : std::iter::Iterator<Item = String> {
    author : User,
    channel : ChannelId,
    command : CommandType,
    args : I,
}

impl Command {
    fn new(author : User, channel : ChannelId, command : CommandType) -> Command {
        Command{author : author, channel : channel, command : command}
    }
    
    //TODO add ways to info.
}

use CommandType::*;
pub enum CommandType {
    NoOp,
    Ping,
    Help,
    LiersDice,
}

fn command_parse(message : Message) -> Result<Command, CommandError> {
    let author = message.author;
    let channel = message.channel_id;

    let mut args = message.content.split(SEPARATOR).filter(|x| *x != SEPARATOR); //This might need... change in the future.

    let command_create = |command_type| {Command::new(author, channel, command_type)};

    if args.next() != Some(PREFIX) {
        return command_create(NoOp);
    }

    let command_type = match args.next() {
        Some(val) => val,
        None => return command_create(Error(CommandError::MissingAction)),
    };

    return command_create(
        match command_type {
            NO_OP_WORD => NoOp,
            HELP_WORD => Help,
            PING_WORD => Ping,
            LIERS_DICE_WORD => todo!(),
    
    
            
            _ => Error(CommandError::NotAction),
        }
    );
}
