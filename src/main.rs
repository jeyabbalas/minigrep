use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Self {
        Self {
            query: args[1].clone(),
            filename: args[2].clone(),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("The input query string is: {}", config.query);
    println!("The file to be searched is: {}", config.filename);

    let contents = fs::read_to_string(config.filename).unwrap();

    println!("File contents:\n{}", contents);
}
