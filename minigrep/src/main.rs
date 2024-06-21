use minigrep::config::Config;
use minigrep::search::str_search;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if (&args).len() < 3 {
        panic!("Usage: `minigrep <pattern> <file>`");
    }
    let config: Config = Config::parse(args);

    for file in config.files {
        let content: String = match fs::read_to_string(&file) {
            Ok(x) => x,
            Err(_) => {
                panic!("Error reading {file}")
            }
        };
        let matches = str_search(&config.query, &content);
        for m in matches {
            print!("{file}\t:{m}\n");
        }
    }
}
