mod parse;
pub use parse::parse;

pub struct Config {
    pub query: String,
    pub files: Vec<String>,
}
impl Config {
    pub fn parse(args: Vec<String>) -> Config {
        parse::parse(args)
    }
}
