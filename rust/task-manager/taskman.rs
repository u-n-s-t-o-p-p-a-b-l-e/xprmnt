use std::env;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

const TASK_FILE: &str = "tasks.txt";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <command> [args]", args[0]);
        std::process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() != 3 {
                eprintln!("Usage: {} add <task>", args[0]);
                std::process::exit(1);
            }
            let task = &args[2];
            if let Err(e) = add_task(task) {
                eprintln!("Error adding task: {}", e);
                std::process::exit(1);
            }
        }
        "list" => {
            if let Err(e) = list_tasks() {
                eprintln!("Error listing tasks: {}", e);
                std::process::exit(1);
            }
        }
        "remove" => {
            if args.len() != 3 {
                eprintln!("Usage: {} remove <task_number>", args[0]);
                std::process::exit(1);
            }
            let task_number: usize = match args[2].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Invalid task number: {}", args[2]);
                    std::process::exit(1);
                }
            };
            if let Err(e) = remove_task(task_number) {
                eprintln!("Error removing task: {}", e);
                std::process::exit(1);
            }
            
        }
        _ => {
            eprintln!("Unknown commnd: {}", command);
            std::process::exit(1);
        }
    }
}


fn add_tasks(task: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).create(true).open(TASKS_FILE)?;
    writeln!(file, "{}", task)?;
    Ok(())

}

fn list_tasks() -> io::Result<()> {
    let mut file = OpenOptions::new().read(true).create(true).open(TASKS_FILE)?;
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        println!("{}: {}", index + 1, line?);
    }
    Ok(())
}

fn remove_task(task_number: usize) -> io::Result<()> {
    let path = Path::new(TASKS_FILE);
    let file = OpenOptions::new().read(true).open(&path)?;
    let reader = BufReader::new(file);

    let tasks: Vec<String> reader.lines().collect::<Result<_, _>>()?;

    if task_number == 0 || task_number > tasks.len() {
        eprintln!("Invalid task number: {}", task_number);
        return Ok(());
    }
}
