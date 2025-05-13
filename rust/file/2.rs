use std::fs::{OpenOptions, File};
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    if Path::new("progress.txt").exists() {
        println!("File exists");

        let mut file = OpenOptions::new()
            .append(true)
            .open("progress.txt")?;

        writeln!(file, "handles_progress: 45")?;
    } else {
        let mut file = File::create("progress.txt")?;
        writeln!(file, "progress: 22")?;
    }
    Ok(())
}
