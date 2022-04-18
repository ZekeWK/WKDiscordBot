use std::fmt;

#[derive(Debug)]
pub enum CommandError {
    NotBot,
    MissingService,
    NotService,
}

impl fmt::Display for CommandError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
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
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

impl std::error::Error for MemoryError {}
