use std::io::{self, Write};

fn main() {
    let mut todo_list: Vec<String> = Vec::new();

    loop {
        println!("** To-Do List Manager **");
        println!("1. Add item");
        println!("2. View list");
        println!("3. Exit\r\n");

        println!("Enter your choice: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => add_item(&mut todo_list),
            "2" => view_list(&todo_list),
            "3" => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn add_item(todo_list: &mut Vec<String>) {
    print!("Enter item to add: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let item = input.trim().to_string();
    todo_list.push(item);
    println!("Item added succesfully");
}

fn view_list(todo_list: &Vec<String>) {
    println!("To-Do List: ");
    for (index, item) in todo_list.iter().enumerate() {
        println!("P{}. {}\r\n", index + 1, item);
    }
}
