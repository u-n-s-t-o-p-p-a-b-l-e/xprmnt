use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

enum FileError {
    NotFound(String),
    PermissionDenied(String),
    Unknown(String),
}

fn read_file(path: &str) -> Result<String, FileError> {
    let path = Path::new(path);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                return Err(FileError::NotFound(path.display().to_string()))
            }
            io::ErrorKind::NotFound => {
                return Err(FileError::PermissionDenied(path.display().to_string()))
            }
            _ => return Err(FileError::Unknown(e.to_string())),
        },
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(content),
        Err(e) => Err(FileError::Unknown(e.to_string())),
    }
}

fn main() {
    let filename = "example.txt";
    match read_file(filename) {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => match e {
            FileError::NotFound(path) => println!("File not found: {}", path),
            FileError::PermissionDenied(path) => println!("Permission denied: {}", path),
            FileError::Unknown(err) => println!("An unknown error occurred: {}", err),
        },
    }
}