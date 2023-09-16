use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn statement_count(&self) -> usize {
        return self.statements.len();
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            return String::new();
        }
        else {
            return self.statements[0].token_literal();
        }
    }
}

pub struct LetStatement {
    pub tok: Token,
    pub name: Identifier,
    pub expr: dyn Expression
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        return self.tok.to_string();
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}


pub struct Identifier {
    tok: Token,
    pub val: String,
}
