use std::process::{Command, Output,Stdio};
use std::io::{self, Read};

fn run_command() -> io::Result<Output> {
    Command::new("ls")
        .arg("-l")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
}

fn main() {
    match run_command() {
        Ok(output) => {
            println!("Command executed successfully.");
            println!("Status: {}", output.status);
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => {
            println!("Failed to execute command: {}", e);
        }
    }
}
