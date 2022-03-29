mod discord;
mod command;
mod task;
mod error;
mod tests;
mod log;

use discord::activate;
use tokio::runtime::Runtime;

fn main() {
    let token = std::env::var("WK_DISCORD_TOKEN").expect("Error getting token.");

    let mut run_time = Runtime::new().unwrap();

    run_time.block_on(activate(&token)).expect("Error creating client.");
}