use std::env;

fn matches(pattern: &str, text: &str) -> bool {
    if pattern.is_empty() {
        return text.is_empty();
    }
}
