use serenity::{
    async_trait,
    model::channel::Message,
    prelude::*,
    Error
};

use crate::base::{command::command_parse, error::CommandError};

struct Handler;

#[async_trait]
impl EventHandler for Handler { //Make it multithread over multiple channels, but not over a single channel as that could lead to some issues during heavy load.
    async fn message(&self, ctx: Context, msg: Message) {
        let (service, command) = match command_parse(&msg) {
            Ok(val) => val,
            Err(CommandError::NotBot) => return,
            Err(CommandError::MissingService) => return,
            _ => unreachable!(),
        };

        let (channel, mut message) = service.execute(command);//Should later allow for multiple messages

        channel.send_message(ctx.http, |_| &mut message).await.unwrap(); //Instead reply message
        //Use a function to Parse message and get a iterator of commands
        //Send to task manager which prepares the message and works with relevant data.
        //Send the message.
        //Will also use the tying marker for when preparing a new message. The start_typing function.
    }
}

pub async fn activate(token : &String) -> Result<(), Error>{    
    Client::builder(&token)
    .event_handler(Handler)
    .await?
    .start()
    .await?;
    Ok(())
}