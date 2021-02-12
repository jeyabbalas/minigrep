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
    let content = fs::read_to_string(config.filename)?;

    let matched = search(&config.query, &content);

    for line in matched {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut matched = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            matched.push(line.trim());
        }
    }

    matched
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let test_query = "who";
        let test_content = "Little owlet in the glen
        I'm ashamed of you;
        You are ungrammatical
        In speaking as you do,
        You should say, \"To whom! To whom!\"
        Not, \"To who! To who\"";
        
        let expected_result = vec!["You should say, \"To whom! To whom!\"",
                                                          "Not, \"To who! To who\""];
        assert_eq!(expected_result, search(test_query, test_content));
    }
}