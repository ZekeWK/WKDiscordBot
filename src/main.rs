#[path ="base.rs"] //In the future one could maybe allow for services to have an init and such...
mod base {
    pub mod discord;
    pub mod command;
    pub mod error;
    pub mod tests;
    pub mod log;
    pub mod service;
    pub mod memory;
    pub mod secret;
}

#[path ="services.rs"]
mod services {
    pub mod liers_dice;
    pub mod info;
    pub mod dice;
}

use base::discord::activate;
use base::secret::OAUTH;
use tokio::runtime::Runtime;

fn main() {
    let run_time = Runtime::new().unwrap();

    run_time.block_on(activate(OAUTH)).expect("Error creating client.");
}