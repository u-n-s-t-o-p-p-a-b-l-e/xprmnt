fn main() {
    let text = "Hi.there";

    let mark: Vec<&str> = text.split('.').collect();

    if let Some(before_dot) = text.get(0) {
        println!("Word before dot: {}", before_dot);
    } else {
        println!("Dot not found.");
    }
    // Other way to do it
    if let Some(mark2) = text.find('.') {
        let before_dot2 = &text[..mark2];
        println!("Word before dot: {}", before_dot);
    } else {
        println!("Dot not found.");
    }
}
