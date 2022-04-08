pub mod ping {
    use crate::base::{service::Service, command::Command, memory::Data};
    use serenity::{model::id::ChannelId, builder::CreateMessage};

    pub const PING_SERVICE : Service = Service{identifier : "ping", handler : ping_handler};

    fn ping_handler(command : Command, data : Option<Data>) -> Vec<(ChannelId, CreateMessage<'static>)> {
        let mut message = CreateMessage::default();
        message.content("Pong!");

        return vec![(command.channel, message)];
    }
}

pub mod help {
    use crate::base::{service::Service, command::Command, memory::Data};
    use serenity::{model::id::ChannelId, builder::CreateMessage};

    pub const HELP_SERVICE : Service = Service{identifier : "help", handler : help_handler};

    fn help_handler(command : Command, data : Option<Data>) -> Vec<(ChannelId, CreateMessage<'static>)> {
        let mut message = CreateMessage::default();
        message.content(
            "Hello! This is a bot by William Kraft. It is currently under heavy development and this is where you will find help later on.");

        return vec![(command.channel, message)];
    }
}

pub mod count {
    use crate::base::{service::Service, command::Command, memory::{Data, to_struct, create_attachment, from_struct}};
    use serenity::{model::id::ChannelId, builder::CreateMessage};

    pub const COUNT_SERVICE : Service = Service{identifier : "count", handler : count_handler};

    fn count_handler(command : Command, data : Option<Data>) -> Vec<(ChannelId, CreateMessage<'static>)> {
        
        let mut num = if let Some(data) = data {
            let num : u64 = to_struct(data);
            num   
        }
        else {
            0u64
        };
        
        num += 1;
        
        let file = create_attachment(from_struct(num), command.msg, COUNT_SERVICE.identifier);
        
        let mut message = CreateMessage::default();
        message.content(format!("You are at : {}", &num)).add_file(file).reference_message(command.message_reference());

        return vec![(command.channel, message)];
    }
}