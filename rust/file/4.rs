use std::io::{self, Write, BufRead, BufReader};
use std::fs::{OpenOptions, File};

fn main() -> io::Result<()> {
    let mut new_line = false;

    {
        let file = File::open("progress.txt")?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if line?.starts_with("handle_progress : ") {
                new_line = true;
                println!("Line already exists");
                break;
            }
        }
    }
    if !new_line {
        let mut file = OpenOptions::new()
            .append(true)
            .open("progress.txt")?;

        writeln!(file, "handle_progress : ")?;
        println!("New line added");
    }
    Ok(())
}
