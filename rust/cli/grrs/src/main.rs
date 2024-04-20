use std::env;
use std::path;

struct Cli {
    pattern: String,
    path: path::PathBuf,
}

fn main() {
    let pattern = env::args().nth(1).expect("no pattern given");
    let path = env::args().nth(2).expect("no path given");

    let args = Cli {
        pattern: pattern,
        path: path::PathBuf::from(path),
    };

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
