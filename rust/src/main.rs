use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    title: String,
    completed: bool,
}


#[derive(Serialize, Deserialize, Debug)]
struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    fn new() -> Self {
        TaskList { tasks: Vec::new() }
    }

    fn add_task(&mut self, title: String) {
        let id = self.tasks.len() + 1;
        let task = Task {
            id,
            title,
            completed: false,
        };
        self.tasks.push(task);
    }

    fn save_to_file(&self, filename: &str) ->  io::Result<()> {
        let file = File::create(filename)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    fn load_from_file(filename: &str) ->  io::Result<Self> {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
        let task_list: TaskList = serde_json::from_reader(reader)?;
        Ok(task_list)
    }
}

fn main() -> io::Result<()> {
    let filename = "tasks.json";

    let mut task_list = match Path::new(filename).exists() {
        true => TaskList::load_from_file(filename)?,
        false => TaskList::new(),
    };

    loop {
        println!("1. Add Task");
        println!("2. Save and Exit");
        print!("> ");

        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => {
                println!("Enter task title: ");
                let mut task_title = String::new();
                io::stdin().read_line(&mut task_title)?;
                task_list.add_task(task_title.trim().to_string());
            }
            "2" => {
                task_list.save_to_file(filename)?;
                println!("Tasks saved to {}", filename);
                break;
            }
            _ => println!("Invalid input"),
        }
    }

    Ok(())
}
