use serenity::{async_trait, model::channel::Message, prelude::*, Error};

use crate::base::{command::{command_parse, command_parse_prefix}, error::CommandError, memory::get_data};

use futures::future::join_all;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {//Will need to make this more pretty.
        let (service, command, data) = match get_data(&ctx.http, &msg.message_reference).await {
            Some((service, data)) => (service, command_parse(&msg), Some(data)),
            
            None => match command_parse_prefix(&msg) {
                Ok((service, command)) => (service, command, None),
                Err(CommandError::NotBot | CommandError::MissingService | CommandError::NotService) => return,
            },
        };      

        let msgs = (service.handler)(command, data);

        let _results = join_all(
            msgs.into_iter()
            .map(|(channel, message)| channel.send_message(&ctx.http, 
                |msg| {*msg = message; return msg;}
            ))).await;
        //Should maybe check for errors?
        //Should maybe look for non vector version.

        //Will also use the tying marker for when preparing a new message. The start_typing function.
        //Similairly might in the future need new command versions to support working with state in threads.
        //Look the filter message.
        //Look into sending multiple requests in one
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