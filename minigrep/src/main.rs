use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config  = Config::new(args);



}

struct Config {
    query: String,
    filename: String,
}

impl Config{
    fn new(args: Vec<String>) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query: query.to_string(), filename: filename.to_string() }
    }
}
