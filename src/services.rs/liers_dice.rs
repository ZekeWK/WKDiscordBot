use std::iter::repeat;
use std::u64;

use crate::base::memory::Data;
use crate::base::{service::Service, command::Command};

use serenity::model::id::{ChannelId, UserId};
use serenity::builder::CreateMessage;

pub const LIERS_DICE_SERVICE : Service = Service{ identifier : "liers", handler : liers_dice_handler};

fn liers_dice_handler(_command : Command, _data : Option<Data>) -> Vec<(ChannelId, CreateMessage<'static>)> {
    todo!()
}

struct LiersDice {
    dice_type : u64,
    players : Vec<(UserId, u64)>,
    last: (u64, u64),    
    turn : usize,
}

impl LiersDice {
    fn new(dice_type : u64, dice_cnt : u64, players : Vec<UserId>) -> LiersDice {
        LiersDice { 
            dice_type: dice_type, 
            players: players.into_iter().zip(repeat(dice_cnt)).collect(), 
            last: (0, 0), //(count, dice)
            turn: 0,
        }
    }

    fn turn(&self) -> UserId {
        self.players[self.turn].0
    }

    fn won(&self) -> bool {
        self.players.iter().filter(|x| x.1 > 0).count() > 1
    }

    fn increase_turn(&mut self) {
        loop {
            self.turn = (self.turn + 1) % self.players.len();
            if self.players[self.turn].1 != 0 {
                break;
            }
        }
    }

    fn raise(&mut self, dice : (u64, u64)) -> Result<(), ()> {
        if dice <= self.last {return Err(())}
        self.last = dice;

        self.increase_turn();

        Ok(())
    }

    fn call() {

    }
    
    
}