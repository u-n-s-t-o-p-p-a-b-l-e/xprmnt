use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
enum MyError {
    Io(io::Error),
    NotFound,
}
