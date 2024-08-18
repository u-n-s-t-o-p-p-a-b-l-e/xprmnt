use std::env;
use std::path::Path;

fn main() {
    let exe_path = env::current_exe().expect("Failed to get executable path");

    let exe_path_str = exe_path.to_str().expect("Failed to convert path to string");

    let file_name = Path::new(exe_path_str)
        .file_name()
        .expect("Failed to get file name")
        .to_str()
        .expect("Failed to convert file name to string");

    println!("File Name: {}", file_name);
    println!("File location: {}", exe_path_str);
}
