use rand::Rng;
use std::io;

fn main() {
    println!("Password Generator");

    let length = get_password_length();

    let password = generate_password(length);
    println!("Generated password: {}", password);
}
