/// mods needed to import in other files.
mod repl;
mod lexer;
mod token;

fn main() {
    println!("Welcome to Lodestone:");
    repl::repl();
}

