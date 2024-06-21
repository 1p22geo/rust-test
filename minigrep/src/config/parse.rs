use crate::config::Config;

pub fn parse(args: Vec<String>) -> Config {
    Config {
        query: args[1].clone(),
        files: args[2..].to_vec(),
    }
}
