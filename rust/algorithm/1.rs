use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);

    let target = "agriculture";

    for line in reader.lines() {
        let line = line?;

        if line.chars().next() == target.chars().next() {
            println!("Same first char: {}:{}", target, line);
            if line.chars().last() == target.chars().last() {
                println!("Same last char: {}:{}", target, line);
                if line == target {
                    println!("Found Match: {}:{}", target, line);
                    break;
                }
            }
        }

    }
    Ok(())
}
