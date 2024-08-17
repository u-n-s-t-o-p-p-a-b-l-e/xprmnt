#[derive(Debug)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    EOF,
}
