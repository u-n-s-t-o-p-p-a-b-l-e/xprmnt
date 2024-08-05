use std::env;

fn matches_here(pattern: &str, text: &str) -> bool {
    if pattern.is_empty() {
        return true;
    }

    if pattern.len() >= 2 && &pattern[1..2] ==  "*" {
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
