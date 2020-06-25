use std::io::{stdin, stdout, Write};

fn output() -> () {
    print!(">>")
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        return l;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespaces(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        } else {
            let ch = self.input.chars().nth(self.read_position).unwrap();
            return ch;
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespaces();
        let token = match self.ch {
            '+' => new_token(TokenType::Plus, self.ch.to_string()),
            '=' => {
                if self.peek_char() == '=' {
                    new_token(TokenType::Equal, self.ch.to_string())
                } else {
                    new_token(TokenType::Assign, self.ch.to_string())
                }
            }
            '(' => new_token(TokenType::Lparen, self.ch.to_string()),
            ')' => new_token(TokenType::Rparen, self.ch.to_string()),
            '{' => new_token(TokenType::Lbrace, self.ch.to_string()),
            '}' => new_token(TokenType::Rbrace, self.ch.to_string()),
            ',' => new_token(TokenType::Comma, self.ch.to_string()),
            '\0' => new_token(TokenType::EOF, self.ch.to_string()),
            _ => new_token(TokenType::Ident, self.ch.to_string()), // TODO Ident or Int
        };

        self.read_char();
        return token;
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Illegal,
    EOF,
    Ident,
    Int,
    Assign,
    Plus,
    Comma,
    SemiColon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
    Equal,
    NotEq,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub typo: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(typo: TokenType, literal: String) -> Self {
        Token { typo, literal }
    }
}

fn new_token(token: TokenType, ch: String) -> Token {
    return Token::new(token.clone(), ch);
}

fn main() {
    let running: bool = true;
    let mut buffer = String::new();
    while running {
        buffer.clear();
        output();

        stdout().flush().unwrap();

        stdin().read_line(&mut buffer).expect("Failed to read");
        if let Some('\n') = buffer.chars().next_back() {
            buffer.pop();
        }

        if let Some('\r') = buffer.chars().next_back() {
            buffer.pop();
        }
        let input = buffer.trim();
        let mut l = Lexer::new(input.to_string());
        loop {
            let tol = l.next_token();
            match tol.typo {
                TokenType::EOF => break,
                _ => println!("{:?} {:?}", tol.typo, tol.literal),
            }
        }

        if input.starts_with(".") {
            match input {
                ".exit" => break,
                _ => (),
            }
        }
    }
}

// TODO write some tests.
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lex() {
        let input = r#"+,"#;

        let l = input.to_string();
        let mut lexer = Lexer::new(l);
        let tokens = vec![(TokenType::Plus, "+"), (TokenType::Comma, ",")];

        for (expected_type, expected_literal) in &tokens {
            let tok = lexer.next_token();
            assert_eq!(tok.typo, *expected_type, "{}", expected_literal);
            assert_eq!(tok.literal, *expected_literal);
        }
    }
}
