use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            query: String::from("query"),
            filename: String::from("filename"),
            case_sensitive: false,
        }
    }
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Implement validation before parsing
        if args.len() < 3 {
            return Err("Not enough arguments supplied expected 3 or 4");
        }
        Config::parse_args(args)
    }

    pub fn parse_args(args: &[String]) -> Result<Config, &'static str> {
        let query = args[1].clone();
        let filename = args[2].clone();
        if args.len() == 3 {
            return Ok(Config {
                query: query,
                filename: filename,
                ..Default::default()
            });
        } else if args.len() == 4 {
            let final_arg = Config::parse_final_arg(args[3].clone());
            match final_arg {
                Err(e) => return Err(e),
                Ok(arg) => {
                    return Ok(Config {
                        query: query,
                        filename: filename,
                        case_sensitive: arg,
                    })
                }
            };
        }
        return Err("Too many arguments supplied expected 3 or 4");
    }

    fn parse_final_arg(arg: String) -> Result<bool, &'static str> {
        let arg_lower = arg.to_lowercase();
        let true_args = vec!["true", "yes", "y"];
        let false_args = vec!["false", "no", "n"];
        if true_args.contains(&arg_lower.as_str()) {
            return Ok(true);
        } else if false_args.contains(&arg_lower.as_str()) {
            return Ok(false);
        } else {
            Err("Final argument must be one of true/yes/y or false/no/n")
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    results.iter().for_each(|line| println!("{}", line));

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents
        .lines()
        .filter(|line| line.contains(query))
        .collect::<Vec<_>>();
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect::<Vec<_>>();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct Tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
