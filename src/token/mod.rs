#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    ILLEGAL(String),
    EOF,

    IDENT(String),
    INT(String),

    ASSIGN,
    PLUS,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET
}

pub fn ident_lookup(ident: &str) -> Token {
    return match ident {
        "fn" => Token::FUNCTION,
        "let" => Token::LET,
        _ => Token::IDENT(ident.to_string())
    }
}
