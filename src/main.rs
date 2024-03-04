use std::io::{self, BufRead};

use clap::Parser;
use regex::Regex;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[arg(short, long)]
    delimiter: String,
    /// field e.g. 1-3 or 3- for everyting from the third field
    #[arg(short, long)]
    field: String,
    /// replace the delimiter with
    #[arg(short, long)]
    replacement: String,
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let args = Cli::parse();
    let delimiter = args.delimiter;
    let replacement = args.replacement;
    let field = args.field;
    let from_to = field.split("-").collect::<Vec<_>>();
    let from_result = from_to[0].parse::<usize>();
    let to_result =
        if from_to.len() == 2 { from_to[1].parse::<usize>() }
        else { from_result.clone() };
    if from_result.is_err() && to_result.is_err() {
        eprintln!("Invalid field: {}", field);
        std::process::exit(1);
    }
    let mut from: usize = 1;
    let mut to = usize::MAX;
    if from_result.is_ok() {
        from = from_result.unwrap();
    }
    if to_result.is_ok() {
        to = to_result.unwrap();
    }
    let re = Regex::new(&delimiter).unwrap();

    for line in stdin.lock().lines() {
        let last_input = line.unwrap();
        let split = re.split(&last_input);
        for (i, e) in split.enumerate() {
            if i >= from - 1 && i <= to - 1 {
                if i != from - 1 {
                    print!("{}", replacement);
                }
                print!("{}", e);
            }
        }
        println!();
    }
    Ok(())
}
