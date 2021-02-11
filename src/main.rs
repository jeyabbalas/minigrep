use std::{env, fs, process};

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Insufficient arguments.");
        }

        let config = Self {
            query: args[1].clone(),
            filename: args[2].clone(),
        };

        Ok(config)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error while parsing input arguments: {}", err);
        process::exit(1);
    });

    println!("The input query string is: {}", config.query);
    println!("The file to be searched is: {}", config.filename);

    let contents = fs::read_to_string(config.filename).unwrap();

    println!("File contents:\n{}", contents);
}
