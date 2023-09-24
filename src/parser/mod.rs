use crate::{lexer::Lexer, token::Token, ast::{Program, Statement, Identifier, Expression, LetStatement}};
use std::mem::discriminant;

#[cfg(test)]
mod tests {
    use crate::ast::Statement;
    use crate::lexer;

    use super::*;

    #[test]
    fn let_statement_test() -> Result<(), String> {
        let input = "\
            let x = 5;\
            let y = 10;\
            let foobar = 838383;\
        ";

        let l = lexer::Lexer::new(input.to_string())?;
        let mut p = Parser::new(l)?;

        let program = p.parse_program()?;
        check_parser_errors(&p)?;
        let statement_count = program.statements.len();
        if statement_count != 3 {
            return Err(format!("Program has wrong statement count! Has {} but \
                               it should be 3!", statement_count));
        }

        let expected_identifier = ["x", "y", "foobar"];
        for (i, ex) in expected_identifier.iter().enumerate() {
            let s = &program.statements[i];
            let res = check_let_statement(s, ex);
            if res.is_err() {
                return res;
            }
        }
        return Ok(());
    }

    fn check_let_statement(s: &Statement, name: &str) -> Result<(), String> {
        match s {
            Statement::LetStatement(s) => {
                if s.tok != Token::LET {
                    return Err(format!("s.tok is not let for a let statement \
                                       got {:?} instead.", s.tok))
                }

                if s.name.val != name {
                    return Err(format!("s.name is not correct for a let statement. \
                                       expected {} but got {} instead.", name, s.name.val));
                }
            },
            _ => return Err(format!("Statement that should be a let statement is \
                                    not, and is {:?} instead.", s))
        }

        return Ok(())
    }

    fn check_parser_errors(p: &Parser) -> Result<(), String> {
        if p.errors.is_empty() {
            return Ok(());
        }
        let e = p.errors.iter().fold("Parser found the following errors:\n".to_string(),
                                     |acc, elem| format!("{}{}\n", acc, elem));
        return Err(e);
    }
}

struct Parser {
    l: Lexer,
    curr_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    fn new(mut l: Lexer) -> Result<Self, String>{
        let tok1 = l.next_token()?;
        let tok2 = l.next_token()?;
        return Ok(Self {
            l,
            curr_token: tok1,
            peek_token: tok2,
            errors: Vec::new()})
    }

    fn next_token(&mut self) -> Result<(), String> {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.l.next_token()?;
        return Ok(())
    }

    fn parse_program(&mut self) -> Result<Program, String> {
        let mut program = Program::new();
        while self.curr_token != Token::EOF {
            let statement = self.parse_statement();
            if let Ok(s) = statement {
                program.statements.push(s);
            }
            self.next_token()?;
        }

        return Ok(program);
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.curr_token {
            Token::LET => return self.parse_let_statement(),
            _ => unimplemented!()
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, String> {
        // parser currtok is a LET token

        if self.expect_peek(&Token::IDENT(String::new())).is_err() {
            return Err(String::new());
        }

        // parser currtok is a IDENT token

        let ident = match &self.curr_token {
            Token::IDENT(name) => Identifier::new(Token::IDENT(name.clone())),
            _ => unreachable!(),
        };

        if self.expect_peek(&Token::ASSIGN).is_err() {
            return Err(String::new());
        }

        // parser currtok is a ASSIGN token


        //skip tokens until a semicolon, we are not parsing expressions yet.
        while !self.curr_token_is(&Token::SEMICOLON) {
            self.next_token()?;
        }

        let st = Statement::LetStatement(LetStatement::new(ident, Expression::new()));
        return Ok(st);
    }

    fn expect_peek(&mut self, expected_tok: &Token) -> Result<(), String> {
        if self.peek_token_is(expected_tok) {
            self.next_token()?;
            return Ok(());
        } else {
            let e = format!("Expected an {} token but got {}",
                                    expected_tok, self.peek_token);
            self.errors.push(e.clone());
            return Err(e);
        }
    }

    fn curr_token_is(&self, expected_tok: &Token) -> bool {
        return discriminant(&self.curr_token) == discriminant(expected_tok)
    }

    fn peek_token_is(&self, expected_tok: &Token) -> bool {
        return discriminant(&self.peek_token) == discriminant(expected_tok)
    }
    // bookmark @ p41
}
