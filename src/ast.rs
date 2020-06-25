use crate::lexer::*;
use crate::parser::*;


#[derive(Debug, Clone)]
pub enum Expression {

}
pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Option<Identifier>,
    pub value: Option<Expression>,
}

impl LetStatement {
    fn statement_node() {
        unimplemented!()
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    fn expression_node() {}
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}

pub trait NodeTrait {
    fn token_literal(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum Statement {
    LetStatement(LetStatement),
}

impl NodeTrait for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::LetStatement(x) => x.token.literal.clone(),
        }
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: vec![],
        }
    }
}

impl NodeTrait for Program {
    fn token_literal(&self) -> String {
        return if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            String::from("")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn statement_test() {
        let input = "+";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 3);

    }
}
