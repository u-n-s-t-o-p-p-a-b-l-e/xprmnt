#[derive(Debug)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    EOF,
}

fn lexer(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' => {
                let mut num = 0;
            }
        }
    }
}
