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
                while let Some('0' ..='9') = chars.peek() {
                    num = num * 10 + chars.next().unwrap().to_digit(10).unwrap() as i32;
                }
                tokens.push(Token::Number(num));
            }
        }
    }
}
