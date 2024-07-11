use std::collections::HashMap;

fn main() {
    let text = "hi world wonderful world";

    let mut word_counts = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }

    for (word, count) in &word_counts {
        println!("{}: {}", word, count);
    }
}
