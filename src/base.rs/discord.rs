use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    Error
};

struct Handler;

#[async_trait]
impl EventHandler for Handler { //Make it multithread over multiple channels, but not over a single channel as that could lead to some issues during heavy load.
    async fn message(&self, ctx: Context, msg: Message) {
        //Use a function to Parse message and get a iterator of commands
        //Send to task manager which prepares the message and works with relevant data.
        //Send the message.
        todo!()
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