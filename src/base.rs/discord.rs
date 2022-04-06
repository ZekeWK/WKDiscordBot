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
            Err(CommandError::NotBot | CommandError::MissingService | CommandError::NotService) => return,
            _ => unreachable!(),
        };

        let (channel, mut message) = service.execute(command);

        channel.send_message(ctx.http, |_| &mut message).await.unwrap(); 
        
        //Should later allow for multiple messages
        //Instead reply to message maybe?
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