use std::process::Command;
use std::str;

fn main() {
    let command = "ls";
    let args = ["-l", "-a"];

   let output = Command::new(command) 
       .args(&args)
       .output()
       .expect("Failed to execute command");
}
