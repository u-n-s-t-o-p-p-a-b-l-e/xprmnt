use std::process::Command;
use std::str;

fn main() {
    let command = "date";

    let output = Command::new(command)
        .output()
        .expect("Failed to execute command");
}
