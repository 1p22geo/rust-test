use std::env;

pub fn arg_collect() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if (&args).len() < 3 {
        panic!("Usage: `minigrep <pattern> <file>`");
    }
    args
}
