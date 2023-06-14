mod token;
mod lexer;
mod repl;
use repl::repl;

fn main() {
    println!("Hello, world!");
    repl();
}
