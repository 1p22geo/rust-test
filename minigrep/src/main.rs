use std::{env, fs};

fn search<'content>(query: &String, content: &'content String) -> Vec<&'content str> {
    let mut found_matches: Vec<&str> = vec![];
    for line in content.split("\n") {
        if line.contains(query) {
            found_matches.push(line)
        }
    }
    found_matches
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if (&args).len() < 3 {
        panic!("Usage: `minigrep <pattern> <file>`");
    }

    let query: String = args[1].clone();
    let files: Vec<String> = args[2..].to_vec();

    for file in files {
        let content: String = match fs::read_to_string(&file) {
            Ok(x) => x,
            Err(_) => {
                panic!("Error reading {file}")
            }
        };
        let matches = search(&query, &content);
        for m in matches {
            print!("{file}\t:{m}\n");
        }
    }
}
