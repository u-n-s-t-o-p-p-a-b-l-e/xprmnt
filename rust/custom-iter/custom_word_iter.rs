struct WordIterator<'a> {
    text: &'a str,
    current_pos: usize,
}

impl<'a> WordIterator<'a> {
    fn new(text: &'a str) -> WordIterator<'a> {
        WordIterator { text, current_pos: 0 }
    }
}

impl<'a> Iterator for WordIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.text = self.text.trim_start();

        if self.text.is_empty() {
            return None;
        }

        let next_space = self.text.fine(' ').unwrap_or(self.text.len());

        let word = &self.text[..next_space];

        self.text = &self.text[next_space..];
        self.current_pos += next_space;

        Some(word)
    }
}
