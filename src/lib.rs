use std::fs;
use std::error::Error;
<<<<<<< HEAD
=======
use std::io;
>>>>>>> second

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
<<<<<<< HEAD
        if args.len() < 3 {
=======
        if args.len() < 4 {
>>>>>>> second
            return Err("Not enough arguments!");
        }

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
<<<<<<< HEAD
            case_sensitive: std::env::var("CASE_INSENSITIVE").is_err(),
        })
    }
=======
            case_sensitive: bool_from_string(args[3].clone()),
        })
    }

    pub fn user_input() -> Result<Config, &'static str> {
        let mut query = String::new();
        let mut filename= String::new();
        let mut case_sensitive= String::new();
    
        println!("Enter the query: ");
        io::stdin().read_line(&mut query).unwrap();
        println!("Enter the filename: ");
        io::stdin().read_line(&mut filename).unwrap();
        println!("Enter the case sensitivity(T/F): ");
        io::stdin().read_line(&mut case_sensitive).unwrap();

        let filename = filename.trim();
        let query = query.trim();
        let case_sensitive = case_sensitive.trim();

        let b = match &case_sensitive[..] {
            "T" => true,
            "F" => false,
            &_ => {return Err("Please, enter T or F");}
        };
    
        Ok(Config {
            query: query.to_string(),
            filename: filename.to_string(),
            case_sensitive: b,
        })
    }
}

fn bool_from_string(s: String) -> bool {
    let b = match &s[..] {
        "T" => true,
        "F" => false,
        &_ => {panic!("Not true of false");}
    };

    b
>>>>>>> second
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
<<<<<<< HEAD
=======

    #[test]
    #[should_panic]
    fn bool_from_string_check_failed() {
        let s = "Q".to_string();
        bool_from_string(s);
    }

    #[test]
    #[should_panic]
    fn bool_from_string_check_failed() {
        let s = "Q".to_string();
        bool_from_string(s);
    }

>>>>>>> second
}
