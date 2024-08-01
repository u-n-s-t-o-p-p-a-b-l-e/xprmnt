fn is_combining_mark(c: char) -> bool {
    match c {
        '\u{0300}'..='\u{036F}' |
        '\u{1AB0}'..='\u{1AFF}' |
        '\u{1DC0}'..='\u{1DFF}' |
        '\u{20D0}'..='\u{20FF}' |
        '\u{FE20}'..='\u{FE2F}' 
        => true,
        _ => false,
    }
}

fn decompose(input: &str) -> Vec<char> {
    let mut decomposed = Vec::new();
    for c in input.chars() {
        decomposed.push(c);

        if is_combining_mark(c) {
            let last_char = decomposed.pop().unwrap();
            decomposed.push('a');
            decomposed.push(last_char);
        }
    }
    decomposed
}

fn recompose(decomposed: Vec<char>) -> String {
    let mut recomposed = String::new();
    for c in decomposed {
        recomposed.push(c);
    }
    recomposed
}

fn normalize(input: &str) -> String {
    let decomposed = decompose(input);
    let recomposed = recompose(decomposed);
    recomposed
}

fn main() {
    let input = "a\u{0301}";
    let normalized = normalize(input);
    println!("Original: {}", input);
    println!("Normalized: {}", normalized);
}
