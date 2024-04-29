use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut tasks: HashMap<String, String> = HashMap::new();

    loop {
        println!("Task Manager");
        println!("1. Add task");
        println!("2. View tasks");
        println!("3. Add Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => add_task(&mut tasks),
            2 => view_tasks(&tasks),
            3 => break,
            _ => println!("Invalid choice. Please enter a number between 1 and 3"),
        }
    }
}

fn add_task(tasks: &mut HashMap<String, String>) {
    print!("Enter task name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    print!("Enter task description: ");
    io::stdout().flush().unwrap();
    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to read input");

    tasks.insert(name.trim().to_string(), description.trim().to_string());
    println!("Task added succesfully");
}

fn view_tasks(tasks: &HashMap<String, String>) {
    println!("Tasks: ");
    for (name, description) in tasks.iter() {
        println!("Name: {}\nDescription: {}\n", name, description);
    }
}
