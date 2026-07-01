use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::search;
use minigrep::search_case_insensitive;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
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
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        //&args[0] returns the program name
        //e.g. running through cargo run returns
        //     the value "target/debug/minigrep"
        args.next();

        let args1 = match args.next() {
            Some(arg) => arg,
            None => return Err("Not enough arguments"),
        };

        let mut ignore_case = env::var("IGNORE_CASE").is_ok();
        let more_args: bool;

        match args1.as_str() {
            "-i" | "--ignore_case" => {
                ignore_case = true;
                more_args = true;
            }
            _ => {
                more_args = false;
            }
        }

        let args2 = match args.next() {
            Some(arg) => arg,
            None => return Err("Not enough arguments"),
        };

        if more_args {
            let args3 = match args.next() {
                Some(arg) => arg,
                None => return Err("Not enough arguments"),
            };

            Ok(Config {
                query: args2,
                file_path: args3,
                ignore_case,
            })
        } else {
            Ok(Config {
                query: args1,
                file_path: args2,
                ignore_case,
            })
        }
    }
}
