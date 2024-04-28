use std::io::{self, Write};

fn main() {
    let mut todos: Vec<String> = Vec::new();

    loop {
        println!("\r\nTodo List Manager\r\n");
        println!("1. Add todo");
        println!("2. View todos");
        println!("3. Remove todo");
        println!("4. Exit");
        println!("5. Enter Your choice: ");

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
            1 => add_todo(&mut todos),
            2 => view_todos(&todos),
             3=> remove_todos(&mut todos),
             4 => break,
             _ => println!("Invalid choice. Please enter a number between 1 and 4"),
        }
    }
}

fn add_todo(todos: &mut Vec<String>) {
    print!(" Enter todo: ");
    io::stdout().flush().unwrap();
    let mut todo = String::new();
    io::stdin().read_line(&mut todo).expect("Failed to read input");
    todos.push(todo.trim().to_string());
    println!("Todo added succesfully");
}

fn view_todos(todos: &Vec<String>) {
    println!("Todos:");
    for (index, todo) in todos.iter().enumerate() {
        println!("{}. {}", index + 1, todo);
    }
}

fn remove_todos(todos: &mut Vec<String>) {
    view_todos(todos);
    if todos.is_empty() {
        println!("No todos to remove.");
        return;
    }

    print!("Enter todo number to remove: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let index: usize = match input.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= todos.len() => num -1,
        _ => {
            println!("Invalid input. Please enter a valid todo number.");
            return;
        }
    };

    let removed_todo = todos.remove(index);
    println!("Removed todo: {}", removed_todo);
}

