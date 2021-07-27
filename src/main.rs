use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> =env::args().collect();
    let config=Config::new(&args).unwrap_or_else(|err| {
        println!("Problem while parsing arguments: {}",err);
        process::exit(1);
    });
    run(config);

}

struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config,&str>{
        if args.len()<3 {
            return Err("Not enough arguments provided!");
        }
        let query=args[1].clone();
        let filename=args[2].clone();
        Ok(Self { query, filename })
    }
}

fn run(config: Config) {
    println!("Searching for {}",config.query);
    println!("In file {}",config.filename);
    println!("----------------------\n");
    
    let contents=fs::read_to_string(config.filename).expect("Could not read file name!");
    println!("Contents of the file are:\n{} ",contents);

}
