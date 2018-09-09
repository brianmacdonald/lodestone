/// mods needed to import in other files.
mod repl;
mod lexer;
mod token;
mod ast;
mod evaluator;
mod environment;
mod parser;
mod object;

fn main() {
    println!("Welcome to Lodestone:");
    repl::repl();
}
