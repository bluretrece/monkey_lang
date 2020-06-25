use crate::ast::*;
use crate::lexer::*;

pub struct Parser {
    pub lexer: Lexer,
    pub current_token: Token,
    pub peek_token: Token,
}

impl Parser {
    pub fn new(l: Lexer) -> Self {
        let mut p = Parser {
            lexer: l,
            current_token: Token::new(TokenType::EOF, "".to_string()),
            peek_token: Token::new(TokenType::EOF, "".to_string()),
        };
        p.next_token();
        p.next_token();
        return p;
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn expect_peek(&mut self, token: TokenType) -> bool {
        if self.peek_token_is(token) {
            self.next_token();
            return true;
        } else {
            return false;
        }
    }

    pub fn peek_token_is(&self, t: TokenType) -> bool {
        return self.peek_token.typo == t;
    }

    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        let mut stmt = LetStatement {
            token: self.current_token.clone(),
            name: None,
            value: None
        };

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        stmt.name = Some(
            Identifier {
                token: self.current_token.clone(),
                value: self.current_token.literal.clone(),
        }
        );

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        while self.current_token_is(TokenType::SemiColon) {
            self.next_token();
        }

        Some(Statement::LetStatement(stmt))
    }

    pub fn current_token_is(&self, current: TokenType) -> bool {
        self.current_token.typo == current
    }


    pub fn parse_statement(&mut self) -> Option<Statement> {
        let stmt = match self.current_token.typo {
            TokenType::Let => self.parse_let_statement(),
            _ => None,
        };

        stmt
    }
    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while self.current_token.typo != TokenType::EOF {
            let stmt = self.parse_statement();

            if stmt.is_some() {
                program.statements.push(stmt.unwrap());
            }

            self.next_token();
        }
        return program;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r"let x = 5;
                          let y = 10;
                          let foobar = 838383;";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        

        assert_eq!(
            program.statements.len(),
            3,
            "Program.statements does not contain 3 statements got={}",
            program.statements.len()
        );

        struct ExpectedIdentifier<'a> {
            value: &'a str,
        }

        let tests = vec![
            ExpectedIdentifier { value: "x" },
            ExpectedIdentifier { value: "y" },
            ExpectedIdentifier { value: "foobar" },
        ];

        for (i, tt) in tests.iter().enumerate() {
            let stmt = match &program.statements[i] {
                Statement::LetStatement(x) => x,
                _ => panic!("Expected letStatement, found {:?}", &program.statements[1]),
            };

            assert_eq!(
                stmt.token.literal,
                String::from("let"),
                "s.token.literal not 'let'. got={}",
                stmt.token.literal
            );

            assert_eq!(
                stmt.name.as_ref().unwrap().value.to_string(),
                tt.value,
                "stmt.Name.Value not'{}'.got={}",
                tt.value,
                stmt.name.as_ref().unwrap().value.to_string()
            );

            assert_eq!(
                stmt.name.as_ref().unwrap().token.literal.to_string(),
                tt.value,
                "s.name not '{}'. got={}",
                tt.value,
                stmt.name.as_ref().unwrap().token.literal.to_string(),
            )
        }
    }
}
