use std::error::Error;
use std::fs;
use std::env;

pub struct Config {

    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config,&str>{
        if args.len()<3 {
            return Err("Not enough arguments provided!");
        }
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        let query=args[1].clone();
        let filename=args[2].clone();
        Ok(Self { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents=fs::read_to_string(config.filename)?;
    println!("Matching lines:\n");

    let results= if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}",line);
    }

    Ok(())

}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query=query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results

}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive() {
        let query="duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.    
Duct tape.";    

        assert_eq!(vec!["safe, fast, productive."],search(query,contents));
    }
    

    #[test]
    fn case_insensitive() {
        let query="rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";    

        assert_eq!(
            vec!["Rust:","Trust me."],
            search_case_insensitive(query,contents)
        );
    }

    #[test]
    fn file_not_exitstent() {
        use super::*;
        let conf=Config {
            query: String::from(""),
            filename: String::from("blablabla"),
            case_sensitive: true,
        };
        assert!(run(conf).is_err());
    }

}
