use std::{env, fs, process, error::Error};

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

        println!("The input query string is: {}", config.query);
        println!("The file to be searched is: {}", config.filename);

        Ok(config)
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("File contents:\n{}", contents);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error while parsing input arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Error while reading input file: {}.", e);
        process::exit(1);
    }
    
}
