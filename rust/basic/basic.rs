use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

enum FileError {
    NotFound(String),
    PermissionDenied(String),
    Unknown(String),
}
