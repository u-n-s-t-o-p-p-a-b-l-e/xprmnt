use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    println!("Enter a string to compute its hash: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let hash = hash_string(&input);

    println!("Hash of '{}' is: {}", input.trim(), hash);

    Ok(())
}

fn hash_string(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}
