use minigrep::{config::Config, search::file_search, util::arg_collect};

fn main() {
    let config: Config = Config::parse(arg_collect());
    for file in config.files {
        for m in file_search(&config.query, &file) {
            print!("{file}\t:{m}\n");
        }
    }
}
