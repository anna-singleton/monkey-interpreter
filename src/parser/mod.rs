use crate::{lexer::Lexer, token::Token, ast::{Program, Statement, LetStatement}};

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
}

struct Parser {
    l: Lexer,
    curr_token: Token,
    peek_token: Token
}

impl Parser {
    fn new(mut l: Lexer) -> Result<Self, String>{
        let tok1 = l.next_token()?;
        let tok2 = l.next_token()?;
        return Ok(Self {
            l,
            curr_token: tok1,
            peek_token: tok2})
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
            if let Some(s) = statement {
                program.statements.push(s);
            }
            self.next_token()?;
        }

        return Ok(program);
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.curr_token {
            Token::LET => return self.parse_let_statement(),
            _ => unimplemented!()
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self
        let s = Statement(LetStatement::new());
        todo!()
    }

    fn expect_peek(&self) -> bool {
        todo!()
    }


    // bookmark @ p41
}
