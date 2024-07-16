fn main() {
    let s = "that";

    match (s.chars().nth(0), s.chars().nth(0)) {
        (Some(first), Some(second)) if first == second => {
            println!("First and second char match.");
        }
        _ => {
            println!("First and second char not match.");
        }
    }
}
