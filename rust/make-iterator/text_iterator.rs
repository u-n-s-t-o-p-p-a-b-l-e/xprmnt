struct TextIterator<'a> {
    text: &'a str,
    position: usize,
}

impl<'a> Iterator for TextIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        while self.position < self.text.len() && self.text.as_bytes()[self.position].is_ascii_whitespace() {
            self.position += 1;
        }

        if self.position >= self.text.len() {
            return None;
        }

        let start = self.position;
        while self.position < self.text.len() && !self.text.as_bytes()[self.position].is_ascii_whitespace() {
            self.position += 1;
        }
        let end = self.position;

        Some(&self.text[start..end])
    }
}
