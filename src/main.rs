use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::search;
use minigrep::search_case_insensitive;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let mut ignore_case = env::var("IGNORE_CASE").is_ok();
        let more_args: bool;

        match args[1].as_str() {
            "-i" | "--ignore_case" => {
                ignore_case = true;
                more_args = true;
            }
            _ => {
                more_args = false;
            }
        }

        if more_args && args.len() < 4 {
            return Err("not enough arguments");
        }

        //&args[0] returns the program name
        //e.g. running through cargo run returns
        //     the value "target/debug/minigrep"
        //let query = &args[1];
        let query = if more_args {
            args[2].clone()
        } else {
            args[1].clone()
        };
        //let file_path = &args[2];
        let file_path = if more_args {
            args[3].clone()
        } else {
            args[2].clone()
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
