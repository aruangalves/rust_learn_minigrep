use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should've been able to read the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        //&args[0] returns the program name
        //e.g. running through cargo run returns
        //     the value "target/debug/minigrep"
        //let query = &args[1];
        let query = args[1].clone();
        //let file_path = &args[2];
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
