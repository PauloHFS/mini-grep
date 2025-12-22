use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub case_sensitive: bool,
}

pub enum GrepError {
    IoError(std::io::Error),
    ConfigError(String),
}

impl Debug for GrepError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigError(error) => f.debug_tuple("Config Error").field(error).finish(),
            Self::IoError(error) => f.debug_struct("Io Error").field("error", error).finish(),
        }
    }
}

impl Display for GrepError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigError(error) => write!(f, "Error de Configuração: {}", error),
            Self::IoError(error) => write!(f, "Error de IO: {}", error),
        }
    }
}

impl Error for GrepError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IoError(error) => Some(error),
            Self::ConfigError(_) => None,
        }
    }
}

impl<'a> Config<'a> {
    pub fn build(file_path: &'a String, query: &'a String) -> Result<Config<'a>, GrepError> {
        let config = Config {
            file_path: file_path,
            query: query,
            case_sensitive: false,
        };
        Ok(config)
    }
}
