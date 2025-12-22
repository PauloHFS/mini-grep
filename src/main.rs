use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use mini_grep::Config;

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

    let file_result = File::open(config.file_path);

    let file = match file_result {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error na configuração: {err}");
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(file);

    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.to_lowercase())
        .filter(|line| line.contains(config.query))
        .for_each(|line| println!("{line}"));
}
