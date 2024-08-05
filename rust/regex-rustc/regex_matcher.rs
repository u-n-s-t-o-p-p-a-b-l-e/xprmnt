use std::env;

fn matches_here(pattern: &str, text: &str) -> bool {
    if pattern.is_empty() {
        return true;
    }
    if pattern.len() >= 2 && &pattern[1..2] == "*" {
        return matches_star(pattern.chars().nth(0).unwrap(), &pattern[2..], text);
    }
    if pattern == "$" {
        return text.is_empty();
    }
    if !text.is_empty() && (pattern.chars().nth(0) == text.chars().nth(0) || pattern.chars().nth(0) == Some('.')) {
        return matches_here(&pattern[1..], &text[1..]);
    }
    false
}

fn matches_star(c: char, pattern: &str, text: &str) -> bool {
    let mut i = 0;
    while i <= text.len() {
        if matches_here(pattern, &text[i..]) {
            return true;
        }
        if i < text.len() && (text.chars().nth(i) == Some(c) || c == '.') {
            i += 1;
        } else {
            break;
        }
    }
    false
}

fn match_pattern(pattern: &str, text: &str) -> bool {
    if pattern.is_empty() {
        return true;
    }
    if pattern.chars().nth(0) == Some('^') {
        return matches_here(&pattern[1..], text);
    }
    let mut i = 0;
    while i <= text.len() {
        if matches_here(pattern, &text[i..]) {
            return true;
        }
        i += 1;
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <pattern> <text>", args[0]);
        std::process::exit(1);
    }

    let pattern = &args[1];
    let text = &args[2];

    if match_pattern(pattern, text) {
        println!("The text matches the pattern.");
    } else {
        println!("The text does not match the pattern.");
    }
}

