use serenity::{async_trait, model::channel::Message, prelude::*, Error};

use crate::base::{command::{command_parse, command_parse_prefix}, error::{CommandError, MemoryError}, memory::get_data, service::ToMessage::*};

use futures::{stream::FuturesOrdered, StreamExt};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {//Will need to make this more pretty.
        
        let reference_information =  match &msg.message_reference {
            Some(msg_ref) => match get_data(&ctx.http, &msg_ref).await {
                Ok(ref_info) => Some(ref_info),
                Err(MemoryError::NotBotData) => None,
                Err(x) => panic!("Error getting data : {}", x),
            },
            None => None,
        };

        let (service, command, data) = 
        if let Some((service, data)) = reference_information {
            (service, command_parse(&msg), Some(data))
        }
        else {
            match command_parse_prefix(&msg) {
                Ok((service, command)) => (service, command, None),
                Err(CommandError::NotBot | CommandError::MissingService | CommandError::NotService) => return,
            }
        };

        let msgs = (service.handler)(command, data);

        let msgs = msgs.into_iter()
            .map(|(to_send, message)| 
            async {
                let to_send = to_send;
                match to_send {
                    User(id) => id.create_dm_channel(&ctx.http).await.unwrap()
                        .send_message(&ctx.http, |msg| {*msg = message; return msg;}).await.unwrap(),
                    Channel(channel) => channel.send_message(&ctx.http, |msg| {*msg = message; return msg;}).await.unwrap(),
                }
            }
        );

        let mut futures = FuturesOrdered::new();

        for msg in msgs {
            futures.push(msg);
        }
    
        let _x = futures.into_future().await; //Look into this
    }
}

pub async fn activate(token : &str) -> Result<(), Error>{    
    Client::builder(token)
    .event_handler(Handler)
    .await?
    .start()
    .await?;
    Ok(())
}