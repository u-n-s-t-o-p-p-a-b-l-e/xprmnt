use std::process::Command;
use std::str;

fn main() {
    let command = "date";

    let output = Command::new(command)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).expect("Failed to parse stdout");
    }
}
