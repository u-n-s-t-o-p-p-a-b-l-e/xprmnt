use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError {
    details: String,
}
