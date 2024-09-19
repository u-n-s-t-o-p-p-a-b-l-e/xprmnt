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

fn add_task() -> io::Result<()> {
    let path = "tasks.txt";

    println!("Enter a task to add:");
    let mut task = String::new();
    io::stdin().read_line(&mut task)?;

    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    writeln!(file, "{}", task.trim())?;

    println!("Task added: {}", task.trim());
    Ok(())
}

fn remove_task() -> io::Result<()> {
    let path = "tasks.txt";

    display_tasks()?;

    println!("Enter the number of the task to remove:");
    let mut task_number = String::new();
    io::stdin().read_line(&mut task_number)?;
    let task_number: usize = task_number.trim().parse().expect("Please enter a valid number");

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let tasks: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    if task_number == 0 || task_number > tasks.len() {
        println!("Invalid task number.");
        return Ok(())
    }

    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;

    for (i, task) in tasks.iter().enumerate() {
        if i + 1 != task_number {
            writeln!(file, "{}", task)?;
        }
    }

    println!("Task {} removed.", task_number);
    Ok(())
}
