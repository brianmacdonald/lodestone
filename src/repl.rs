use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fs::File;

use super::lexer::Lexer;
use super::parser::Parser;
use super::evaluator::eval;
use super::environment::Environment;

pub fn repl() {
    let filename = "./lodestone/lobby.ldst";
    let mut f = File::open(filename).expect("file not found");
    let mut env = Environment{store: HashMap::new()};
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let il = Lexer::new(contents);
    let mut ip = Parser::new(il);
    let iprogram = ip.parse_program();
    eval(iprogram, &mut env);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(
        "Failed to read line",
    );
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    let evaluated = eval(program, &mut env);
    println!("You inputted: {}", evaluated);
}
