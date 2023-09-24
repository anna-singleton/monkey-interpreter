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
    ReturnStatement(ReturnStatement),
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub expr: Expression,
}

impl ReturnStatement {
    pub fn new(expr: Expression) -> Self {
        return ReturnStatement { expr }
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub tok: Token,
    pub name: Identifier,
    expr: Expression,
}

impl LetStatement {
    pub fn new(i: Identifier, e: Expression) -> Self {
        return Self{
            tok: Token::LET,
            name: i,
            expr: e};
    }
}

#[derive(Debug)]
pub struct Identifier {
    tok: Token,
    pub val: String,
}

impl Identifier {
    pub fn new(tok: Token) -> Self {
        let val = match &tok {
            Token::IDENT(s) => s.clone(),
            _ => panic!("a token that is not IDENT was passed to create an identifier"),
        };
        return Self {
            tok,
            val,
        }
    }
}

#[derive(Debug)]
pub struct Expression {

}

impl Expression {
    pub fn new() -> Self {
        return Expression {};
    }
}
