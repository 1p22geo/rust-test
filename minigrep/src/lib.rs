pub mod util;

pub mod search {
    use std::fs;

    pub fn str_search(query: &String, content: &String) -> Vec<String> {
        let mut found_matches: Vec<String> = vec![];
        for line in content.split("\n") {
            if line.contains(query) {
                found_matches.push(line.to_owned())
            }
        }
        found_matches
    }
    pub fn file_search(query: &String, filename: &str) -> Vec<String> {
        let content: String = match fs::read_to_string(&filename) {
            Ok(x) => x,
            Err(_) => {
                panic!("Error reading {filename}")
            }
        };
        str_search(&query, &content)
    }
}

pub mod config {
    pub struct Config {
        pub query: String,
        pub files: Vec<String>,
    }
    impl Config {
        pub fn parse(args: Vec<String>) -> Config {
            Config {
                query: args[1].clone(),
                files: args[2..].to_vec(),
            }
        }
    }
}
