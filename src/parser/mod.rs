use crate::{lexer::Lexer, token::Token, ast::Program};

#[cfg(test)]
mod tests {
    use crate::{ast::LetStatement, ast::Node};
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

        let program = p.parse_program();
        let statement_count = program.statement_count();
        if statement_count != 3 {
            return Err(format!("Program has wrong statement count! Has {} but \
                               it should be 3!", statement_count));
        }

        let expected_identifier = ["x", "y", "foobar"];
        for (i, ex) in expected_identifier.iter().enumerate() {

        }
        todo!()
    }

    fn check_let_statement(s: &LetStatement, name: &str) -> Result<(), String> {
        if s.token_literal() != "let" {
            return Err(format!("s.token_literal is not let for a let statement \
                               got {} instead.", s.token_literal()))
        }

        if s.name.val != name {
            return Err(format!("s.name is not correct for a let statement. \
                               expected {} but got {} instead.", name, s.name.val));
        }

        todo!()
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

    // bookmark @ p39

    fn next_token(&mut self) -> Result<(), String> {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.l.next_token()?;
        return Ok(())
    }

    fn parse_program(&mut self) -> Program {
        todo!()
    }
}
