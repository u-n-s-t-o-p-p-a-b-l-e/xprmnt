#[derive(Debug)]
enum ASTNode {
    Function(String, Vec<String>),
    If,
    Else,
    While,
    Identifier(String),
}

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

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            current_char: None,
        };
        lexer.read_char();
        lexer
    }
}
