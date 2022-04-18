use std::fmt;

#[derive(Debug)]
pub enum CommandError {
    NotBot,
    MissingService,
    NotService,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CommandError::NotBot => write!(f, "Not a bot command."),
            CommandError::NotService => write!(f, "Not a service."),
            CommandError::MissingService => write!(f, "Missing service identifier."),
        }
    }
}

impl std::error::Error for CommandError {}

#[derive(Debug)]
pub enum MemoryError {
    NotMessage,
    NotBotData,
    NotService,
    NotChannel,
    FailDownload,
    FailDecrypt,
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            MemoryError::NotMessage => write!(f, "Did not reply to a message."),
            MemoryError::NotService => write!(f, "No service file."),
            MemoryError::NotBotData => write!(f, "Data not by bot."),
            MemoryError::NotChannel => write!(f, "Message reference not sent in a channel."),
            MemoryError::FailDownload => write!(f, "Download failed."),
            MemoryError::FailDecrypt => write!(f, "Decryption failed."),
        }
    }
}

impl std::error::Error for MemoryError {}
