use std::env;

/// mods needed to import in other files.
mod repl;
mod lexer;
mod token;
mod ast;
mod evaluator;
mod environment;
mod parser;
mod object;
mod builtins;

fn main() {
    let args: Vec<String> = env::args().collect();
    repl::repl(args.get(1));
}
