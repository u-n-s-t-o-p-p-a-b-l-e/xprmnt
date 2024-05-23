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

impl Tasklist {
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
}
