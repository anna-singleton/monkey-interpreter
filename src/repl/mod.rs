use std::io::{self, Write};
use crate::lexer::Lexer;

const PROMPT:&str = "-> ";


pub fn repl() -> Result<(), String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        print!("{}", PROMPT);
        stdout.flush();
        if let Err(e) = stdin.read_line(&mut buffer) {
            return Err(format!("couldnt read from stdin! error: {}\nquitting...", e).to_string())
        }

        let mut l = Lexer::new(buffer.split_off(0))?;

        let mut tokens = vec![];

        loop {
            match l.next_token() {
                Ok(tok) => {
                    match tok {
                        crate::token::Token::EOF => break,
                        _ => tokens.push(tok),
                    }
                },
                Err(e) => return Err(format!("error reading tokens: {}\nquitting.", e).to_string()),
            }
        }
        println!("{:?}", tokens);
    }
}
