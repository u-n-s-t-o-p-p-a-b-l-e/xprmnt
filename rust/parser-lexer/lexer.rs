#[derive(Debug, PartialEq)]
enum Token {
    Identifier(String),
    Keyword(String),
    LParen,
    RParen,
    Comma,
    Eof,
}

struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    current_char: Option<char>,
}
