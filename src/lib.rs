use std::env;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(file_path: &'a str, query: &'a str) -> Result<Config<'a>, GrepError> {
        if query.is_empty() {
            return Err(GrepError::ConfigError(
                "O parâmetro 'query' não pode ser vazio".to_string(),
            ));
        }
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            file_path,
            query,
            ignore_case,
        })
    }
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

pub fn grep(config: &Config, mut writer: impl std::io::Write) {
    let file_result = File::open(config.file_path);

    let file = match file_result {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error na configuração: {err}");
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(file);

    if config.ignore_case {
        let query_lower = config.query.to_lowercase();
        reader
            .lines()
            .map_while(Result::ok)
            .enumerate()
            .filter(|(_i, line)| line.to_lowercase().contains(&query_lower))
            .for_each(|(i, line)| writeln!(writer, "[LINHA {}]: {}", i + 1, line).unwrap());
    } else {
        reader
            .lines()
            .map_while(Result::ok)
            .enumerate()
            .filter(|(_i, line)| line.contains(config.query))
            .for_each(|(i, line)| writeln!(writer, "[LINHA {}]: {}", i + 1, line).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_build_success() {
        let config = Config::build("./file.txt", "query").unwrap();
        assert_eq!(config.file_path, "./file.txt");
        assert_eq!(config.query, "query");
        // Por padrão, IGNORE_CASE não está setado, então deve ser false
        assert!(!config.ignore_case);
    }

    #[test]
    fn test_config_build_empty_query() {
        let result = Config::build("./file.txt", "");
        assert!(result.is_err());
        if let Err(GrepError::ConfigError(msg)) = result {
            assert_eq!(msg, "O parâmetro 'query' não pode ser vazio");
        } else {
            panic!("Esperado ConfigError, mas ocorreu outro tipo de erro.");
        }
    }
}
