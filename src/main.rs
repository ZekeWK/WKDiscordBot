#[path ="base.rs"] //Maybe make games into traits?
mod base {
    pub mod discord;
    pub mod command;
    pub mod task;
    pub mod error;
    pub mod tests;
    pub mod log;
}

#[path ="games.rs"]
mod games {
    pub mod liers_dice;
}

use base::discord::activate;
use tokio::runtime::Runtime;

fn main() {
    let token = std::env::var("WK_DISCORD_TOKEN").expect("Error getting token.");

    let mut run_time = Runtime::new().unwrap();

    run_time.block_on(activate(&token)).expect("Error creating client.");
}