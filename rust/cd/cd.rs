use std::env;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
       eprintln!("usage: {} <directory>", args[0]); 
       process::exit(1);
    }
    
    let new_dir = &args[1];
    if let Err(e) = env::set_current_dir(Path::new(new_dir)) {
        eprintln!("Failed to change directory: {}", e);
        process::exit(1);
    }

    println!("Changed directory to {}", new_dir);
}
