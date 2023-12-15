#[cfg(test)]
mod tests;

pub mod config;
mod search;

use std::{error::Error, fs};

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let res = search::search(&config.query, &contents);
    for line in res {
        println!("{}", line)
    }

    Ok(())
}
