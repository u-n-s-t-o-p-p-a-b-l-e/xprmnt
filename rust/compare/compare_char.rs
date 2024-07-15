fn main() {
    let s = "that";

    if let (Some(first), Some(second)) = (s.chars().nth(0), s.chars().nth(1)) {
        if first == second {
            println!("First and second char are the same.");
        } else {
            println!("First and second char are not the same.");
        }
    }
}
