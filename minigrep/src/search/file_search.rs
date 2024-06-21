use crate::search::str_search;
use std::fs;

pub fn file_search(query: &String, filename: &str) -> Vec<String> {
    let content: String = match fs::read_to_string(&filename) {
        Ok(x) => x,
        Err(_) => {
            panic!("Error reading {filename}")
        }
    };
    str_search(&query, &content)
}
