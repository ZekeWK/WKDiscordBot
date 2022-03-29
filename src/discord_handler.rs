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
        todo!()
    }
}

pub async fn start_bot(token : &String) -> Result<(), Error>{    
    Client::builder(&token)
    .event_handler(Handler)
    .await?
    .start()
    .await?;
    Ok(())
}