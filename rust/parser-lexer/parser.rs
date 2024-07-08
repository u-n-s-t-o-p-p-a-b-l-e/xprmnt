#[derive(Debug)]
enum ASTNode {
    Function(String, Vec<String>),
    If,
    Else,
    While,
    Identifier(String),
}
