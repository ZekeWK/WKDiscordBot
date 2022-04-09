use crate::base::{service::Service, command::Command, memory::Data};
use serenity::{model::id::ChannelId, builder::CreateMessage};

pub const DICE_SERVICE : Service = Service{identifier : "dice", handler : dice_handler};

//"3d4 + - 4d2332 + 32"



fn dice_handler(command : Command, data : Option<Data>) -> Vec<(ChannelId, CreateMessage<'static>)> { //Not good code.
    
    todo!()
}




/* fn parse_dice(command : Command) ->  Vec<(i64, u64)>{
    let input = command.args.collect::<String>();

    let rolls : Vec<(i64, u64)> = Vec::new();
    let mut sum = 0i64;

    let mut unproccessed = input.as_str();

    loop {
        let mut positive = true;

        for (i, c) in unproccessed.chars().enumerate() {
            match c {
                '+' => (),
                '-' => positive = !positive,
                _ => { unproccessed = unproccessed.split_at(i).1; break;},
            }
        }

        for (i, c) in unproccessed.chars().enumerate() {
            match c {
                '0'..='9' => (),
                '-' | '+' => {
                    let (number, unproccessedt) = unproccessed.split_at(i);
                    unproccessed = unproccessedt;

                    let number = number.parse();
                    rolls.append(());
                },
                'd' => { unproccessed = unproccessed.split_at(i).1; break;},
                _ => todo!()
            }
        }
    }   
}
 */
    /*
    let mut rolls = command.args.map(|x| x.split("+").filter(|x| *x != ""));
    
    let mut msg = String::new();
    
    for roll in rolls {
        for die in roll {
            let (cnt, val) = match die.split_once("d") {
                Some(cnt, ) => todo!(),
                _ => unreachable!()
            }
        }
    }
    */


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
 