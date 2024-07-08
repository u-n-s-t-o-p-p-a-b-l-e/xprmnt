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
            Token::Keyword(identifier.to_string())
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

struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn parse(&mut self) -> Vec<ASTNode> {
        let mut nodes = Vec::new();

        while self.current_token != Token::Eof {
            match &self.current_token {
                Token::Keyword(keyword) => match keyword.as_str() {
                    "fn" => nodes.push(self.parse_function()),
                    "if" => {
                        nodes.push(ASTNode::If);
                        self.next_token();
                    }
                    "else" => {
                        nodes.push(ASTNode::Else);
                        self.next_token();
                    }
                    "while" => {
                        nodes.push(ASTNode::While);
                        self.next_token();
                    }
                    _ => panic!("Unexpected keyword: {}", keyword),
                },
                Token::Identifier(identifier) => {
                    nodes.push(ASTNode::Identifier(identifier.clone()));
                    self.next_token();
                }
                _ => panic!("Unexpected token: {:?}", self.current_token),
            }
        }

        nodes
    }

    fn parse_function(&mut self) -> ASTNode {
        self.next_token(); // Skip 'fn'

        if let Token::Identifier(name) = &self.current_token {
            let function_name = name.clone();
            self.next_token(); // Skip function name

            self.expect_token(Token::LParen);

            let mut parameters = Vec::new();
            while self.current_token != Token::RParen {
                if let Token::Identifier(param) = &self.current_token {
                    parameters.push(param.clone());
                    self.next_token();
                }

                if self.current_token == Token::Comma {
                    self.next_token();
                }
            }

            self.expect_token(Token::RParen);

            ASTNode::Function(function_name, parameters)
        } else {
            panic!("Expected function name, found {:?}", self.current_token);
        }
    }

    fn expect_token(&mut self, token: Token) {
        if self.current_token == token {
            self.next_token();
        } else {
            panic!(
                "Expected token: {:?}, found: {:?}",
                token, self.current_token
            );
        }
    }
}

fn main() {
    let input = "fn example(a, b) if else while";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    println!("{:?}", ast);
}
