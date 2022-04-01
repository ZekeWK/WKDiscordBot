use std::io::Error;
use std::fmt;

pub enum CommandError {
    NotBot,
    MissingService,
    NotService,
    MissingArgs,
    UnvalidArgs,
}

impl fmt::Debug for CommandError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

impl std::error::Error for CommandError {

}