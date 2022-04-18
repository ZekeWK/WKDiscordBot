use crate::base::{service::service_prelude::*, command::Command};

use serde::{Serialize, Deserialize};
use serenity::model::{id::UserId};
use serenity::builder::CreateMessage;

use rand::distributions::{Distribution, Uniform};

//Make it send to people that they are dead...
//



pub const LIARS_DICE_SERVICE : Service = Service{ identifier : "liars", handler : liars_dice_handler};

fn liars_dice_handler(mut command : Command, data : Option<Data>) -> Vec<(ToMessage, CreateMessage<'static>)> {
    match command.args.next() {
        Some("new") => {
            let liars_dice = liars_dice_from_args(command.author, &mut command.args); 

            if liars_dice.is_none() {
                let mut message = CreateMessage::default();
                message.content("Error parsing.").reference_message(command.message_reference());

                return vec![(Channel(command.channel), message)];
            }

            let liars_dice = liars_dice.unwrap();

            let mut msgs = Vec::new();

            send_dice(&liars_dice, &mut msgs);

            let attachment = attachment_from_struct(liars_dice, command.msg, LIARS_DICE_SERVICE.identifier);

            let mut message = CreateMessage::default();
            message.content(format!("Started a new game! <@{}>, it is your turn!", command.author)).add_file(attachment).reference_message(command.message_reference());
            msgs.push((Channel(command.channel), message));


            return msgs;

        },
        Some("raise") => {
            if data.is_none() {
                let mut message = CreateMessage::default();
                message.content("Did not provide data.").reference_message(command.message_reference());

                return vec![(Channel(command.channel), message)];
            }

            let mut liars_dice : LiarsDice = to_struct(data.unwrap());

            if liars_dice.turn() != command.author {
                let mut message = CreateMessage::default();
                message.content(format!("It is not your turn. It is <@{}> 's turn", liars_dice.turn())).reference_message(command.message_reference());
                return vec![(Channel(command.channel), message)];
            }
            

            if liars_dice_raise(&mut liars_dice, &mut command.args).is_none() {
                let mut message = CreateMessage::default();
                message.content("You need to raise!").reference_message(command.message_reference());

                return vec![(Channel(command.channel), message)];
            }

            let mut message = CreateMessage::default();
            message.content(format!("You have raised! <@{}>, it is your turn!", liars_dice.turn()));

            let attachment = attachment_from_struct(liars_dice, command.msg, LIARS_DICE_SERVICE.identifier);
            message.add_file(attachment).reference_message(command.message_reference());
    
            return vec![(Channel(command.channel), message)];
        },
        Some("call") => {
            if data.is_none() {
                let mut message = CreateMessage::default();
                message.content("Did not provide data.").reference_message(command.message_reference());

                return vec![(Channel(command.channel), message)];
            }

            let mut liars_dice : LiarsDice = to_struct(data.unwrap());

            if liars_dice.turn() != command.author {
                let mut message = CreateMessage::default();
                message.content(format!("It is not your turn. It is <@{}> 's turn", liars_dice.turn())).reference_message(command.message_reference());
                return vec![(Channel(command.channel), message)];
            }

            let result = liars_dice.call();

            if let Some(winner) = liars_dice.won() {
                let mut message = CreateMessage::default();
                message.content(format!("The winner is: <@{}>", winner.to_string())).reference_message(command.message_reference());
                return vec![(Channel(command.channel), message)];
            }

            let mut msg = if result {
                let mut msg = CreateMessage::default();
                msg.content(format!("The dice did not exist! Starting next round with <@{}>.", liars_dice.turn())).reference_message(command.message_reference());
                msg
            }
            else {
                let mut msg = CreateMessage::default();
                msg.content(format!("The dice did exist! Starting next round with <@{}>", liars_dice.turn())).reference_message(command.message_reference());
                msg
            };

            let mut msgs = Vec::new();

            send_dice(&liars_dice, &mut msgs);
            
            let attachment = attachment_from_struct(liars_dice, command.msg, LIARS_DICE_SERVICE.identifier);

            msg.add_file(attachment).reference_message(command.message_reference());
            
            msgs.push((Channel(command.channel), msg));
            
            return msgs;
        },
        _ => {
            let mut message = CreateMessage::default();
            message.content("Not a command.").reference_message(command.message_reference());
            return vec![(Channel(command.channel), message)];
        }
    }
}



 

fn liars_dice_from_args(author : UserId, args : &mut ArgsIter) -> Option<LiarsDice> {
    let mut players = vec![author.0];    

    let mut next = args.next();
    
    while next.is_some() && next.unwrap().starts_with("<@") && next.unwrap().ends_with(">") {
        let user : UserId = next.unwrap().parse().ok()?;
        players.push(user.0);

        next = args.next();
    }

    let (dice_type, dice_cnt) = if next.is_some() {
        let (dice_cnt, dice_type) = next.unwrap().split_once("d")?;
        let dice_type : usize = dice_type.parse().ok()?;
        let dice_cnt : usize = dice_cnt.parse().ok()?;
        (dice_type, dice_cnt)
    }
    else {(6, 5)};

    Some(LiarsDice::new(dice_type, dice_cnt, players))
}

fn liars_dice_raise(liars_dice : &mut LiarsDice, args : &mut ArgsIter) -> Option<()> {
    let (dice_cnt, dice) = match (args.next()?, args.next()?, args.next()?) {
        (dice_cnt, "of", dice) => (dice_cnt, dice),
        _ => return None,
    };

    let dice : usize = dice.parse().ok()?;
    let dice_cnt : usize = dice_cnt.parse().ok()?;

    liars_dice.raise(dice_cnt, dice)
}

fn send_dice(liars_dice : &LiarsDice, vec : &mut Vec<(ToMessage, CreateMessage<'static>)>) {
    for i in 0..liars_dice.players.len() {
        let user = liars_dice.players[i].0;

        let dice = liars_dice.get_dice(i);
        
        let mut message = CreateMessage::default();
        message.content(format!("Your dice are : {:?}.", dice));

        vec.push((User(UserId(user)), message));
    }
}


#[derive(Serialize, Deserialize)]
struct LiarsDice {
    dice_type : usize,
    players : Vec<(u64, usize)>,
    last: (usize, usize),
    turn : usize,
    dice : Vec<usize>,
}

impl LiarsDice {
    fn new(dice_type : usize, dice_cnt : usize, players : Vec<u64>) -> LiarsDice {
        let len = players.len();
        let mut liars_dice = LiarsDice { 
            dice_type: dice_type, 
            players: players.into_iter().map(|user| (user, dice_cnt)).collect(), 
            last: (0, 0), //(count, dice)
            turn: 0,
            dice : vec![0; len*dice_cnt],
        };
        liars_dice.roll();
        liars_dice
    }

    fn turn(&self) -> UserId {
        UserId(self.players[self.turn].0)
    }

    fn last_turn(&self) -> usize {
        let mut i = self.turn;
        loop {
            i = (i + self.players.len() - 1) % self.players.len();
            if self.players[i].1 != 0 {
                return i;
            }
        }
    }

    fn won(&self) -> Option<UserId> {
        let mut alive = self.players.iter().filter(|x| x.1 > 0);

        let first_alive = alive.next().unwrap().0;
    
        if alive.next().is_none(){
            return Some(UserId(first_alive));
        }
        else {
            return None;
        }
    }

    fn increase_turn(&mut self) {
        loop {
            self.turn = (self.turn + 1) % self.players.len();
            if self.players[self.turn].1 != 0 {
                break;
            }
        }
    }

    fn raise(&mut self, dice_cnt : usize, dice : usize) -> Option<()> {
        if (dice_cnt, dice) <= self.last {return None}
        self.last = (dice_cnt, dice);

        self.increase_turn();

        Some(())
    }

    fn call(&mut self) -> bool { //Bool is if it was a false raise.
        let real_cnt = (0..self.players.len())
            .map(|i| self.get_dice(i)
            .into_iter()).flatten()
            .filter(|die| **die == self.last.1)
            .count();

        let result = real_cnt<self.last.0;
        
        if result {
            let last_turn = self.last_turn();
            self.players[last_turn].1 -= 1;
        }
        else {
            let turn = self.turn;
            self.players[turn].1 -= 1;
        }
        
        self.roll();
        self.increase_turn();

        result
    }

    fn get_dice(&self, user_index : usize) -> &[usize] {
        let start_i = (self.dice.len() / self.players.len()) * user_index;
        let stop_i = start_i + self.players[user_index].1;

        &self.dice[start_i..stop_i]
    }
    
    fn roll(&mut self) {
        let between = Uniform::from(1usize..=self.dice_type);
        let mut rng = rand::thread_rng();
        self.last = (0, 0);

        for i in self.dice.iter_mut() {
            *i = between.sample(&mut rng);
        }
    }
    
}