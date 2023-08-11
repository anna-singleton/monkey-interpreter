mod token;
mod lexer;
mod repl;
mod ast;

use repl::repl;

fn main() {
    println!("Hello, world!");
    repl();
}
