use std::fs::{OpenOptions, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn display_tasks() -> io::Result<()> {
    let path = "tasks.txt";

    if Path::new(&path).exitst() {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        println!("Your To-Do List:");
        for (i, line) in reader.lines().enumerate() {
            println!("{}. {}", i + 1, line?);
        }
    } else {
        println!("No tasks available. Add a task to get started.");
    }

    Ok(())
}
