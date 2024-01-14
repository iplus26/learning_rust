#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String
}

use text_colorizer::*;
use std::env;
use std::fs;
use regex::Regex;

fn print_usage() {
    eprintln!("{} - change occurrences of one string to another", "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <filename> <output>");
}

fn replace(target: &str, replacement: &str, data: &str) -> Result<String, regex::Error> {
    let re = Regex::new(target)?;
    Ok(re.replace_all(data, replacement).to_string())
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        print_usage();
        eprintln!("{}: wrong number of arguments", "quickreplace".red());
        std::process::exit(1);
    }

    let target = args[1].clone();
    let replacement = args[2].clone();
    let filename = args[3].clone();
    let output = args[4].clone();

    Arguments { target, replacement, filename, output }
}

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", "quickreplace".red(), e);
            std::process::exit(1);
        }
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", "quickreplace".red(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(_) => println!("{}: {}", "quickreplace".green(), "success"),
        Err(e) => {
            eprintln!("{}: {}", "quickreplace".red(), e);
            std::process::exit(1);
        }
    }

    println!("{:?}", args);
}
