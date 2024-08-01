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

dn decompose(input: &str) -> Vec<char> {
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
