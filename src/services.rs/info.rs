pub mod ping {
    use crate::base::{service::Service, command::Command};
    use serenity::{model::id::ChannelId, builder::CreateMessage};

    pub const PING_SERVICE : Service = Service{identifier : "ping", handler : ping_handler};

    fn ping_handler(command : Command) -> (ChannelId, CreateMessage<'static>) {
        let mut message = CreateMessage::default();
        message.content("Pong!");

        return (command.channel, message);
    }
}

pub mod help {
    use crate::base::{service::Service, command::Command};
    use serenity::{model::id::ChannelId, builder::CreateMessage};

    pub const HELP_SERVICE : Service = Service{identifier : "help", handler : help_handler};

    fn help_handler(command : Command) -> (ChannelId, CreateMessage<'static>) {
        let mut message = CreateMessage::default();
        message.content(
            "Hello! This is a bot by William Kraft. It is currently under heavy development and this is where you will find help later on.");

        return (command.channel, message);
    }
}