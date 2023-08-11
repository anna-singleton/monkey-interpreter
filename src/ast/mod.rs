use crate::token::Token;

trait Node {
    fn token_literal(&self) -> String;
}

trait Statement: Node {
    fn statement_node(&self);
}

trait Expression: Node {
    fn expression_node(&self);
}

struct Program {
    statements: Vec<Box<dyn Statement>>,
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

struct LetStatement {
    tok: Token,
    name: Identifier,
    expr: dyn Expression
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        return self.tok.to_string();
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}


struct Identifier {
    tok: Token,
    val: String,
}
