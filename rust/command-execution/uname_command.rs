use std::process::Command;
use std::str;

fn main() {
    let command = "uname";
    let args = ["-a"];

    let output = Command::new(command)
        .args(&args)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).expect("Failed to parse stdout");
        println!("System information:\n{}", stdout);
    } else {
        let stderr = str::from_utf8(&output.stderr).expect("Failed to parse stderr");
        println!("Command failed:\n{}", stderr);
    }
}
