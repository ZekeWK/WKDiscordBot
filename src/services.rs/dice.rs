use crate::base::{service::Service, command::Command};
use serenity::{model::id::ChannelId, builder::CreateMessage};

pub const DICE_SERVICE : Service = Service{identifier : "dice", handler : dice_handler};

//"3d4 + - 4d2332 + 32"



fn dice_handler(command : Command) -> (ChannelId, CreateMessage<'static>) { //Not good code.
/*    let mut args = &mut command.args.iter().map(|x| x.chars()).flatten().peekable();

    let mut answer = String::new();

    let success : bool = 'outer : loop {
        let mut positive = true;
        loop {
            match args.peek() {
                Some('+') => {args.next().unwrap();},
                Some('-') => {args.next().unwrap(); positive = !positive},
                None => break 'outer true,
                _ => break,
             };
        }

        let mut first_number = String::new();

        let is_constant = loop {
            match args.peek() {
                Some('0'..='9') => first_number.push(args.next().unwrap()),
                Some('d') => {args.next(); break false;}
                Some('+' | '-') => break true,
                None => break 'outer true,
                _ => break 'outer false,
            }
        };

        if is_constant {

        };


        loop {
            match args.peek() {
                Some('0'..='9') => first_number.push(args.next().unwrap()),
                Some('+' | '-') => break,
                None => break,
                _ => break 'outer false,
            }
        }
        



    };
    

    
*/    
    
    let message = todo!();
    
    return (command.channel, message);
}