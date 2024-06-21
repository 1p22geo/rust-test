pub mod search {
    pub fn str_search<'content>(query: &String, content: &'content String) -> Vec<&'content str> {
        let mut found_matches: Vec<&str> = vec![];
        for line in content.split("\n") {
            if line.contains(query) {
                found_matches.push(line)
            }
        }
        found_matches
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
