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
    }
}
