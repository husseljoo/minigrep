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

    // println!("Searching for {}",config.query);
    // println!("In file {}",config.filename);
    // println!("----------------------\n");
    println!("Contents of the file are:\n{} ",contents);

    Ok(())

}
