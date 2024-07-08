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

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = Some(self.input.as_bytes()[self.read_position] as char);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.current_char {
            Some('(') => {
                self.read_char();
                Token::LParen
            }
            Some(')') => {
                self.read_char();
                Token::RParen
            }
            Some(',') => {
                self.read_char();
                Token::Comma
            }
            Some(c) if c.is_alphabetic() => self.read_identifier(),
            None => Token::Eof,
            _ => panic!("Unexpected character: {:?}", self.current_char),
        };

        token
    }

    fn read_identifier(&mut self) -> Token {
        let position = self.position;
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() {
                self.read_char();
            } else {
                break;
            }
        }
        let identifier = &self.input[position..self.position];
        if self.is_keyword(identifier) {
            Token::keyword(identifier.to_string())
        } else {
            Token::Identifier(identifier.to_string())
        }
    }

    fn is_keyword(&self, identifier: &str) -> bool {
        matches!(identifier, "if" | "else" | "while" | "fn")
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }
}

fn main() {
    let input = "fn example(a, b) if else while";
    let mut lexer = Lexer::new(input);

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == Token::Eof {
            break;
        }
    }
}
