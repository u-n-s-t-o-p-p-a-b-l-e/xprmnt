use std::process::{Command, Output,Stdio};
use std::io::{Self, Read};

fn run_command() -> io::Result<Output> {
    Commandd::new("ls")
        .arg("-l")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
}
