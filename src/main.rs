use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_arg(&args);

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Failed to read file");

    println!("With given text\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_arg(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
