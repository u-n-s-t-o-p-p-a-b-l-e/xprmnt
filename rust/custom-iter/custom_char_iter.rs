struct Chariterator<'a> {
    text: &'str,
    current_pos: usize,
}

impl<'a> CharIterator<'a> {
    fn new(text: &'a str) -> CharIterator<'a> {
        CharIterator { text, current_post: 0 }
    }
}

impl<'a> CharIterrator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos >= self.text.len() {
            return None;
        }

        let ch = self.text[self.current_pos..].chars().next().unwrap();
        let char_len = ch.len_utf8();

        self.current_pos += char_len;

        Some(ch)
    }
}

fn main() {
    lt input = "Rust is great!";

    let mut char_iter = CharIterator::new(input);

    while let Some(ch) = char_iter.next() {
        println!("Character: {}", ch);
    }
}
