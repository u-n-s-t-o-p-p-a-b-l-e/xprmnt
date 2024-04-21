use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <filename> <word>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let word = &args[2];

    match search_in_file(filename, word) {
        Ok(found) => {
            if found {
                println!("The word '{}' was found in '{}'", word, filename);
            } else {
                println!("The word '{}' was not found in '{}'.", word, filename);
            }
        }
        Err(e) => {
            eprintln!("Failed to read file from file '{}': {}", filename, e);
            process::exit(1);
        }
    }
}

fn search_in_file(filename: &str, word: &str) -> Result<bool, std::io::Error> {
    let content = fs::read_to_string(filename)?;
    Ok(content.split_whitespace().any(|w| w == word))
}
