mod token;
mod lexer;
mod repl;
mod ast;
mod parser;

use repl::repl;

fn main() {
    println!("Hello, world!");
    repl();
}
