use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open("progress.txt")?;
    let reader = BufReader::new(file);
    let mut progress = 0;

    for line in reader.lines() {
        let line = line?;

        if line.starts_with("progress : ") {
            if let Some(value_str) = line.strip_prefix("progress : ") {
                if let Ok(value) = value_str.trim().parse::<i32>() {
                    progress = value
                }
            }
        }
    }
    println!("Progress: {}", progress);
    Ok(())

}
