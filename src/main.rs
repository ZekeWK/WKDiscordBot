#[path ="base.rs"] //In the future one could maybe allow for services to have an init and such...
mod base {
    pub mod discord;
    pub mod command;
    pub mod error;
    pub mod tests;
    pub mod log;
    pub mod service;
    pub mod memory;
}

#[path ="services.rs"]
mod services {
    pub mod liers_dice;
    pub mod info;
}

use base::discord::activate;
use tokio::runtime::Runtime;

fn main() {
    let token = std::env::var("WK_DISCORD_TOKEN").expect("Error getting token.");

    let run_time = Runtime::new().unwrap();

    run_time.block_on(activate(&token)).expect("Error creating client.");
}