use std::{env, process};
use minigrep_iter::Config;
use minigrep_iter;

fn main(){
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments {}", err);
        process::exit(1);
    });

    println!("search {} in {}", config.query, config.file_path);

    if let Err(e) = minigrep_iter::run(config) {
        eprintln!("Application error {}", e);
        process::exit(1);
    }

}
