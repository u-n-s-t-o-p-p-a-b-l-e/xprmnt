use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError {
    details: String,
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()} 
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn perform_action(succeed: bool) -> Result<(), Box<dyn Error>> {
    if succeed {
        Ok(())
    } else {
        Err(Box::new(MyError::new("Something went wrong")))
    }
}

