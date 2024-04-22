use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };

    if let Err(e) = list_directory_contents(path) {
        eprintln!("Error listing contents of directory: {}", e);
        process::exit(1);
    }
}

fn list_directory_contents(path: &str) -> Result<(), std::io::Error> {
    let path = Path::new(path);
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let file_name = entry.file_name().into_string().unwrap_or_else(|_| "Invalid UTF-8 name".to_string());

            if file_type.is_dir() {
                println!("{}/", file_name);
            } else {
                println!("{}", file_name);
            }
        }
    } else {
        println!("{} is not a directory", path.display());
    }
    Ok(())
}
