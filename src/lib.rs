use std::{fs, env, error::Error};

pub struct Config {
    query: String,
    filename: String,
    case_insensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Insufficient arguments.");
        }

        let config = Self {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_insensitive: !env::var("CASE_INSENSITIVE").is_err(),
        };

        println!("The input query string is: {}", config.query);
        println!("The file to be searched is: {}\n", config.filename);

        Ok(config)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let matched = if config.case_insensitive {
        case_insensitive_search(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in matched {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut matched = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            matched.push(line.trim());
        }
    }

    matched
}

fn case_insensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut matched = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
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
        Not, \"To who! To who!\"";
        
        let expected_result = vec!["You should say, \"To whom! To whom!\"",
                                                          "Not, \"To who! To who!\""];
        assert_eq!(expected_result, search(test_query, test_content));
    }

    #[test]
    fn test_case_insensitive_search() {
        let query = "YOU";
        let content = "Little owlet in the glen
        I'm ashamed of you;
        You are ungrammatical
        In speaking as you do,
        You should say, \"To whom! To whom!\"
        Not, \"To who! To who!\"";

        let expected_result = vec!["I'm ashamed of you;",
                                "You are ungrammatical",
                                "In speaking as you do,",
                                "You should say, \"To whom! To whom!\""];

        assert_eq!(expected_result, case_insensitive_search(query, content));
    }

    #[test]
    fn test_case_sensitive_search() {
        let query = "You";
        let content = "Little owlet in the glen
        I'm ashamed of you;
        You are ungrammatical
        In speaking as you do,
        You should say, \"To whom! To whom!\"
        Not, \"To who! To who!\"";

        let expected_result = vec!["You are ungrammatical",
                                "You should say, \"To whom! To whom!\""];

        assert_eq!(expected_result, search(query, content));
    }
}