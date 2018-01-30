/// mods needed to import in other files.
mod repl;
mod lexer;
mod token;
mod ast;

fn main() {
    println!("Welcome to Lodestone:");
    repl::repl();
}
