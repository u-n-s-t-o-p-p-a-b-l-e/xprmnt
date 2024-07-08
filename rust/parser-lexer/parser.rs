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
