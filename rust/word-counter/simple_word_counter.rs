use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_path = &args[1];

    if !Path::new(file_path).exists() {
        println!("Error: File '{}' does not exits.", file_path);
        return;
    }

    match count_words_in_file(file_path) {
        Ok(count) => println!("The file contains {} words.", count),
        Err(e) => println!("Error reading file: {}", e),
    }
}

fn count_words_in_file(file_path: &str) -> io::Result<usize> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut word_count = 0;

    for line in reader.lines() {
        let line  = line?;
        word_count += line.split_whitespace().count();
    }

    Ok(word_count)
}
