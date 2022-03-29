mod discord_handler;

use discord_handler::start_bot;
use tokio::runtime::Runtime;

fn main() {
    let token = std::env::var("WK_DISCORD_TOKEN").expect("Error getting token.");

    let mut run_time = Runtime::new().unwrap();

    run_time.block_on(start_bot(&token)).expect("Error creating client.");
}