use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    ILLEGAL(String),
    EOF,

    IDENT(String),
    INT(String),

    ASSIGN,
    PLUS,
    MINUS,
    SLASH,
    ASTERISK,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
    IF,
    ELSE,
    RETURN,

    EQ,
    NOT_EQ,
    LT,
    GT,
    BANG,
    TRUE,
    FALSE,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:?}", self)
    }
}

pub fn ident_lookup(ident: &str) -> Token {
    return match ident {
        "fn" => Token::FUNCTION,
        "let" => Token::LET,
        "true" => Token::TRUE,
        "false" => Token::FALSE,
        "if" => Token::IF,
        "else" => Token::ELSE,
        "return" => Token::RETURN,
        _ => Token::IDENT(ident.to_string())
    }
}
