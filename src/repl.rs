use std::io;
use std::collections::HashMap;

use super::lexer::Lexer;
use super::parser::Parser;
use super::evaluator::eval;
use super::environment::Environment;

pub fn repl() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(
        "Failed to read line",
    );
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let mut program = p.parse_program();
    let evaluated = eval(program, &mut Environment{store: HashMap::new()});
    println!("You inputted: {}", evaluated);
}
