use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <pattern> <filename>", args[0]);
        std::process::exit(1);
    }

    let pattern = &args[1];
    let filename = &args[1];

    let re = Regex::new(pattern).expect("Invalid regex pattern");

    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        if re.is_match(&line) {
            println!("{} found in {}", re, line);
        }
    }

    Ok(())
}
