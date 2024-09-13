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
    }
}
