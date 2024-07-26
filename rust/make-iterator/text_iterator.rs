struct TextIterator<'a> {
    text: &'a str,
    position: usize,
}

impl<'a> Iterator for TextIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {}
}
