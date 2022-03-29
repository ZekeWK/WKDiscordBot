use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    Error
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        //Use a function to Parse message and get a iterator of commands
        //Send to task manager
        
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