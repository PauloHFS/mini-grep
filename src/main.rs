use std::env;

use mini_grep::{Config, grep};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    let config_result = Config::build(file_path, query);

    let config = match config_result {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Erro na configuração: {err}");
            std::process::exit(1);
        }
    };

    let stdout = std::io::stdout();
    grep(&config, stdout);
}
