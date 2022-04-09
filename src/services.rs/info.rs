pub mod ping {

    use crate::base::service::service_prelude::*;

    pub const PING_SERVICE : Service = Service{identifier : "ping", handler : ping_handler};

    fn ping_handler(command : Command, _data : Option<Data>) -> Vec<(ToMessage, CreateMessage<'static>)> {
        /*
        if let Some(message) = ping_annoy(&mut command) {
            return  message;
        }
        else {*/return ping_std(&command);//};

    }
    fn ping_std(command : &Command) -> Vec<(ToMessage, CreateMessage<'static>)> {
        let mut message = CreateMessage::default();
        message.content("Pong!").reference_message(command.message_reference());
        
        return vec![(Channel(command.channel), message)];
    }

/*
    fn ping_annoy(command : &mut Command) -> Option<Vec<(ChannelId, Vec<(ToMessage, CreateMessage<'static>)>>{
        let user_str : &str = command.args.next()?;
        if !user_str.starts_with("<@") || !user_str.ends_with(">") {
            return None
        }
        
        let user : UserId = user_str.parse().ok()?;
        let cnt : usize = command.args.next().unwrap_or("5").parse().ok()?;
        
        if cnt > 20 {
            let mut message = CreateMessage::default();
            message.content("To many! The max is 20.").reference_message(command.message_reference());
            return Some(vec![(command.channel, message)]);
        }
        let mut message = CreateMessage::default();
        message.content(format!("Ping! <@{}>", user));
        
        let messages : Vec<(ChannelId, CreateMessage<'static>)> = repeat_with(|| (command.channel ,message.clone())).take(cnt).collect();
        
        Some(messages)
    }
*/
}

pub mod help {
    use crate::base::service::service_prelude::*;

    pub const HELP_SERVICE : Service = Service{identifier : "help", handler : help_handler};

    fn help_handler(command : Command, _data : Option<Data>) -> Vec<(ToMessage, CreateMessage<'static>)> {
        let mut message = CreateMessage::default();
        message.content(
            "You will find help here later on."
        ).reference_message(command.message_reference());

        return vec![(Channel(command.channel), message)];
    }
}

pub mod info {
    use crate::base::service::service_prelude::*;

    pub const INFO_SERVICE : Service = Service{identifier : "info", handler : info_handler};

    fn info_handler(command : Command, _data : Option<Data>) -> Vec<(ToMessage, CreateMessage<'static>)> {
        let mut message = CreateMessage::default();
        message.content(
            "<@958421014063239209> is a bot by William Kraft. It is currently under heavy development, but the code can be found at https://github.com/ZekeWK/WKDiscordBot ."
        ).reference_message(command.message_reference());

        return vec![(Channel(command.channel), message)];
    }
}

pub mod count {
    use crate::base::service::service_prelude::*;

    pub const COUNT_SERVICE : Service = Service{identifier : "chain", handler : count_handler};

    fn count_handler(command : Command, data : Option<Data>) -> Vec<(ToMessage, CreateMessage<'static>)> {
        
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
        message.content(format!("You are link {} in the chain!", &num)).add_file(file).reference_message(command.message_reference());

        return vec![(Channel(command.channel), message)];
    }
}