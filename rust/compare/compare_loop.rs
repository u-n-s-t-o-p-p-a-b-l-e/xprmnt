fn main() {
    let s = "that";

    let mut chars = s.chars();

    if let Some(first) = chars.next() {
        for c in chars {
            if first == c {
                println!("Found the same char: {}", c);
            } else {
                println!("Not found. First char: {} | Next char: {}", first, c);
            }
        }
    }
}
