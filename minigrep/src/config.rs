pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn parse(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough parameters");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Self { query, file_path })
    }
}
