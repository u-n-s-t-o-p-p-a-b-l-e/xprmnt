use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    loop {
        println!("1. Add note");
        println!("2. View notes");
        println!("3. Exit");
        println!("Enter your choice: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number");
                continue;
            }
        };

        match choice {
            1 => add_note(),
            2 => view_notes(),
            3 => break,
            _ => println!("Invalid choice. Please enter a number between 1 and 3"),
        }
    }
}

fn add_note() {
    println!("Enter your note (press Enter to finish)");

    let mut note = String::new();
    io::stdin().read_line(&mut note).expect("Failed to read input");

    let mut file = match File::create("notes.txt") {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to create or open notes file");
            return;
        }
    };

    if let Err(_) = writeln!(file, "{}", note) {
        println!("Failed to write to notes file");
        return;
    }

    println!("Note added succesfully");

}

fn view_notes() {
    println!("Notes:");

    let file = match File::open("notes.txt") {
        Ok(file) => file,
        Err(_) => {
            println!("No notes found");
            return;
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(note) = line {
            println!("{}", note);
        }
    }

} 
