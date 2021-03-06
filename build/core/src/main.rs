#![crate_name = "cosmos"]
use std::env;
use std::fs;

fn main() {
    println!("Welcome to Cosmos!");
    file();
}

fn file() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for: {}", config.query);
    println!("In: {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("Containing: {}", contents)
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
