use std::env;
use std::fs;

fn main() {
    let args: Vec<String> =env::args().collect();
    let config = parse_config(&args);
    display_content(config);
}

struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Self{
        let query=args[1].clone();
        let filename=args[2].clone();
        Self { query, filename }
    }
}

fn parse_config(args: &[String]) -> Config {
    Config::new(args)
}


fn display_content(config: Config) {
    println!("Searching for {}",config.query);
    println!("In file {}",config.filename);
    println!("----------------------\n");
    
    let contents=fs::read_to_string(config.filename).expect("Could not read file name!");
    println!("Contents of the file are:\n{} ",contents);

}
