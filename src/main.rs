use minigrep::{search, search_case_insensitive};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    // main gathers raw input (args)
    // main handles startup failure (bad args)
    // run assumes it has a valid configuration

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results: Vec<_> = if config.ignore_case {
        search_case_insensitive(&config.query, &contents).collect()
    } else {
        search(&config.query, &contents).collect()
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
        // call next first to ignore program name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case =
        // args contains --ignore-case OR we check if the env var is set to 1, true, yes or on.
        // args.contains being first means that explicitly writing it in overrides env vars.
            args.any(|arg| arg == "--ignore-case")
            || match std::env::var("IGNORE_CASE") {
                Ok(val) => matches!(
                    val.as_str(),
                    "1" | "true" | "yes" | "on"
                ),
                Err(_) => false,
            };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
