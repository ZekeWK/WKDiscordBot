use serenity::{async_trait, model::channel::Message, prelude::*, Error};

use crate::base::{command::command_parse, error::CommandError, memory::parent_data};

use futures::future::join_all;
struct Handler;

#[async_trait]
impl EventHandler for Handler { //Make it multithread over multiple channels, but not over a single channel as that could lead to some issues during heavy load.
    async fn message(&self, ctx: Context, msg: Message) {
        let (service, command) = match command_parse(&msg) {
            Ok(val) => val,
            Err(CommandError::NotBot | CommandError::MissingService | CommandError::NotService) => return,
        };

        let data = parent_data(&ctx.http, &msg.message_reference, service.identifier).await;

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