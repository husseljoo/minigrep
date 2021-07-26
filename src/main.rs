use std::env;
use std::fs;

fn main() {
    let args: Vec<String> =env::args().collect();
    let (query, filename) = parse_config(&args);
    display_content(query,filename);
}


fn parse_config(args: &[String]) -> (&str, &str) {
    let query=&args[1];
    let filename=&args[2];
    (query,filename)

}


fn display_content(query: &str, filename: &str) {
    println!("Searching for {}",query);
    println!("In file {}",filename);
    println!("----------------------\n");
    
    let contents=fs::read_to_string(filename).expect("Could not read file name!");
    println!("Contents of the file are:\n{} ",contents);

}
