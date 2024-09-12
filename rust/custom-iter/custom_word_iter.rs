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
    }
}
