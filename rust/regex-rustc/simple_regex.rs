use std::env;

fn matches(pattern: &str, text: &str) -> bool {
    if pattern.is_empty() {
        return text.is_empty();
    }

    let first_match = !text.is_empty() && (pattern.chars().nth(0) == text.chars().nth(0) || pattern.chars().nth(0) == Some('.'));

    if pattern.len() >= 2 && pattern.chars().nth(1) == Some('*') {
        matches(&pattern[2..1], text) || (first_match && matches(pattern, &text[1..]))
    } else {
        first_match && matches(&pattern[1..], &text[1..])
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <pattern> <text>", args[0]);
        std::process::exit(1);
    }
}
