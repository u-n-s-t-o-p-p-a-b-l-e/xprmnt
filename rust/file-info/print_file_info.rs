use std::path::Path;

fn main() {
    let exe_path = env::current_exe().expect("Failed to get executable path");

    let exe_path_str = exe_path.to_str().expect("Failed to convert path to string");
}
