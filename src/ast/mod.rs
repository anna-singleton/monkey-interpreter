use crate::token::Token;

#[derive(Debug)]
pub enum Node {
    Statement(Statement),
    Expression(Expression),
    Program(Program),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>
}

impl Program {
    pub fn new() -> Self {
        return Program { statements: Vec::new() };
    }
}

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
}

#[derive(Debug)]
pub struct LetStatement {
    pub tok: Token,
    pub name: Identifier,
    expr: Expression,
}

#[derive(Debug)]
pub struct Identifier {
    tok: Token,
    pub val: String,
}

#[derive(Debug)]
pub enum Expression {

}
