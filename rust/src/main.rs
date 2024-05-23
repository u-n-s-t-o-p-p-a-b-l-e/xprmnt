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


