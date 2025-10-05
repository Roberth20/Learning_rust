use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::{search, search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();

    // And exits with nonzero value if fail
    let config = Config::build(&args).unwrap_or_else(|err| {
        // eprintln! allow to print with standard error stream instead of standard output
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for query '{}'", config.query);
    println!("In file '{}'", config.file_path);

    // Error handling for run function
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

// Extract the running process from main
fn run(config: Config) -> Result<(), Box<dyn Error>>{
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

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // Not appropiate, better use Result
            // panic!("Not enough arguments");
            return Err("Not enoug arguments");
        }
        // Clone is not the appropiate to do it by performance and memory management
        let query = args[1].clone();
        let file_path = args[2].clone();

        // Checking for environment variable
        let ignore_case = if args.len() == 4 {
            true
        } else {
            env::var("IGNORE_CASE").is_ok()
        };
        // Case as argument with precedence

        Ok(Config{query, file_path, ignore_case})
    }
}