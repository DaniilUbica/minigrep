use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive: std::env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

pub fn read_file(config: &Config) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)?;

    Ok(content)
}

pub fn search<'a> (query: &str, content: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut v: Vec<&'a str> = vec![];

    let q: String;

    if !case_sensitive {
        q = query.to_lowercase();
    }
    else {
        q = query.to_string();
    }

    for i in content.lines() {
        if !case_sensitive {
            if i.to_lowercase().contains(&q) {
                v.push(i);
            }
        }
        else {
            if i.contains(&q) {
                v.push(i);
            }
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_check_sensitive() {
        let query = "Safe";
        let content = "\
        Safe, fast, productive.
        pick three.";
        assert_eq!(vec!["Safe, fast, productive."], search(query, content, true));
    }

    #[test]
    fn search_check_insensitive() {
        let query = "Safe";
        let content = "\
        safe, fast, productive.
        pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content, false));
    }
}
