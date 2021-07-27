use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config,&str>{
        if args.len()<3 {
            return Err("Not enough arguments provided!");
        }
        let query=args[1].clone();
        let filename=args[2].clone();
        Ok(Self { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents=fs::read_to_string(config.filename)?;
    println!("Matching lines:\n");
    for line in search(&config.query, &contents) {
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


#[cfg(test)]
mod tests{

    #[test]
    fn one_result() {
        use super::*;
        let query="duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";    

        assert_eq!(vec!["safe, fast, productive."],search(query,contents));
    }
    


    #[test]
    fn file_not_exitstent() {
        use super::*;
        let conf=Config {
            query: String::from(""),
            filename: String::from("blablabla"),
        };
        assert!(run(conf).is_err());
    }

}
