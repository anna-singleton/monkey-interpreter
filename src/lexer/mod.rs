use crate::token::{Token, ident_lookup};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_next_token_test() {
        let input = "=+(){},;".to_string();
        let expected_output = vec![
		Token::ASSIGN,
		Token::PLUS,
		Token::LPAREN,
		Token::RPAREN,
		Token::LBRACE,
		Token::RBRACE,
		Token::COMMA,
		Token::SEMICOLON,
		Token::EOF,
        ];
        let mut l = Lexer::new(input).expect("AAAAAAH NON ASCII, FUUUUUCK");

        for expected in expected_output.iter() {
            let tok = l.next_token();
            assert!(tok.is_ok());
            assert_eq!(tok.unwrap(), *expected);
        }
    }

    #[test]
    fn next_token_with_ident_test() {
        let input = "let five = 5;\
                     let ten = 10;\
                     let add = fn(x, y) {\
                       x + y;\
                     };\
                     \
                     let result = add(five, ten);".to_string();
        let expected_output = vec![
            Token::LET,
            Token::IDENT("five".to_string()),
            Token::ASSIGN,
            Token::INT("5".to_string()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".to_string()),
            Token::ASSIGN,
            Token::INT("10".to_string()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".to_string()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_string()),
            Token::COMMA,
            Token::IDENT("y".to_string()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".to_string()),
            Token::PLUS,
            Token::IDENT("y".to_string()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".to_string()),
            Token::ASSIGN,
            Token::IDENT("add".to_string()),
            Token::LPAREN,
            Token::IDENT("five".to_string()),
            Token::COMMA,
            Token::IDENT("ten".to_string()),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::EOF

        ];
        let mut l = Lexer::new(input).expect("AAAAAAH NON ASCII, FUUUUUCK");

        for expected in expected_output.iter() {
            let tok = l.next_token();
            assert!(tok.is_ok());
            assert_eq!(tok.unwrap(), *expected);
        }
    }

    #[test]
    fn extended_next_token_test() {
        let input = "let five = 5;\
            let ten = 10;\
            let add = fn(x, y) {\
              x + y;\
            };\
            \
            let result = add(five, ten);\
            !-/*5;\
            5 < 10 > 5;\
            if (5 < 10) {\
                return true;\
            } else {\
                return false;\
            }\
            \
            10 == 10;\
            10 != 9;".to_string();
        let expected_output = vec![
            Token::LET,
            Token::IDENT("five".to_string()),
            Token::ASSIGN,
            Token::INT("5".to_string()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".to_string()),
            Token::ASSIGN,
            Token::INT("10".to_string()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".to_string()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_string()),
            Token::COMMA,
            Token::IDENT("y".to_string()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".to_string()),
            Token::PLUS,
            Token::IDENT("y".to_string()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".to_string()),
            Token::ASSIGN,
            Token::IDENT("add".to_string()),
            Token::LPAREN,
            Token::IDENT("five".to_string()),
            Token::COMMA,
            Token::IDENT("ten".to_string()),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT("5".to_string()),
            Token::SEMICOLON,
            Token::INT("5".to_string()),
            Token::LT,
            Token::INT("10".to_string()),
            Token::GT,
            Token::INT("5".to_string()),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT("5".to_string()),
            Token::LT,
            Token::INT("10".to_string()),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::INT("10".to_string()),
            Token::EQ,
            Token::INT("10".to_string()),
            Token::SEMICOLON,
            Token::INT("10".to_string()),
            Token::NOT_EQ,
            Token::INT("9".to_string()),
            Token::SEMICOLON,
            Token::EOF,
        ];
        let mut l = Lexer::new(input).expect("AAAAAAH NON ASCII, FUUUUUCK");

        for expected in expected_output.iter() {
            let tok = l.next_token();
            assert!(tok.is_ok());
            assert_eq!(tok.unwrap(), *expected);
        }
    }
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char
}

impl Lexer{
    pub fn new(input: String) -> Result<Self, String> {
        if !input.is_ascii() {
            return Err("Non ASCII detected! Unsupported for now :( (for ever)".to_string());
        }
        let mut l = Self { input, position: 0, read_position: 0, ch: '\0' };
        l.read_char();
        return Ok(l);
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.eat_whitespace();
        let tok = match self.ch {
			'=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::EQ
                }
                else {
                    Token::ASSIGN
                }
            },
			'+' => Token::PLUS,
            '-' => Token::MINUS,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NOT_EQ
                }
                else {
                    Token::BANG
                }
            },
            '*' => Token::ASTERISK,
            '/' => Token::SLASH,
			'(' => Token::LPAREN,
			')' => Token::RPAREN,
			'{' => Token::LBRACE,
			'}' => Token::RBRACE,
			',' => Token::COMMA,
			';' => Token::SEMICOLON,
            '<' => Token::LT,
            '>' => Token::GT,
			'\0' => Token::EOF,
            _ => {
                if is_ident_char(self.ch) {
                    // early return because this has already advanced the
                    // position, we dont need to advance again.
                    return Ok(ident_lookup(&self.read_ident()));
                }
                else if self.ch.is_numeric() {
                    return Ok(Token::INT(self.read_number()))
                }
                else {
                    Token::ILLEGAL(self.ch.to_string())
                }
            }
        };
        self.read_char();
        return Ok(tok);
    }

    pub fn read_char(&mut self) {
        self.ch = if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_number(&mut self) -> String {
        let start_pos = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        return self.input[start_pos..self.position].to_string();
    }

    fn read_ident(&mut self) -> String {
        let start_pos = self.position;
        while is_ident_char(self.ch) {
            self.read_char();
        }
        return self.input[start_pos..self.position].to_string();
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        }
        else {
            return self.input.chars().nth(self.read_position).unwrap();
        }
    }

    fn eat_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

}

fn is_ident_char(c: char) -> bool {
    if c.is_alphabetic() {
        return true;
    }

    if c == '_' {
        return true;
    }

    return false;
}
