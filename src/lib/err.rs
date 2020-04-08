use std::error::Error;
use std::fmt;
#[derive(Debug)]
pub enum CliError {
    File(std::io::Error),
    Parse(std::num::ParseIntError),
    Time(std::time::SystemTimeError),
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> CliError {
        CliError::File(err)
    }
}

impl From<std::num::ParseIntError> for CliError {
    fn from(err: std::num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}

impl From<std::time::SystemTimeError> for CliError {
    fn from(err: std::time::SystemTimeError) -> CliError {
        CliError::Time(err)
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::File(ref err) => write!(f, "File error: {}", err),
            CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
            CliError::Time(ref err) => write!(f, "Time error: {}", err),
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CliError::File(err) => Some(err),
            CliError::Parse(err) => Some(err),
            CliError::Time(err) => Some(err),
        }
    }
}
