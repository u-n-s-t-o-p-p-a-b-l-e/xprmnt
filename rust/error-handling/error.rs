use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
enum MyError {
    Io(io::Error),
    NotFound,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::Io(ref err) => write!(f, "IO error: {}", err),
            MyError::NotFound => write!(f, "File not found"),
        }
    }
}
