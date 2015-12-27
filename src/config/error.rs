use std::error::Error as StdError;
use std::fmt;
use std::string;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Parse,
    InvalidGroup,
    NotifiersMissing,
    NotifiersInvalid,
    NotifierInvalid,
    NotifierUrlMissing,
    NotifierUrlInvalid
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Parse => "failed to parse toml: {:?}",
            Error::InvalidGroup => "every config group must be a table: {:?}",
            Error::NotifiersMissing => "no notifiers provided",
            Error::NotifiersInvalid=> "notifiers must be an array",
            Error::NotifierInvalid => "a notifier must be a table",
            Error::NotifierUrlMissing => "a notifier must have a url",
            Error::NotifierUrlInvalid => "a notifier must have a valid url: {:?}"
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
