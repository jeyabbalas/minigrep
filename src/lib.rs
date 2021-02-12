use std::{fs, error::Error};

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Insufficient arguments.");
        }

        let config = Self {
            query: args[1].clone(),
            filename: args[2].clone(),
        };

        println!("The input query string is: {}", config.query);
        println!("The file to be searched is: {}", config.filename);

        Ok(config)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("File contents:\n{}", contents);

    Ok(())
}