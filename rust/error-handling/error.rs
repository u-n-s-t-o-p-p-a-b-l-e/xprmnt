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

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static')> {
        match *self {
            MyError::Io(ref err) => Some(err),
            MyError::NotFound => None,
        }
    }
}

impl From<io::Error> for MyError {
    fn from(err: io::Error) -> MyError {
        MyError::Io(err)
    }
}

fn read_file_contents(file_path: &str) -> Result<String, MyError> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
