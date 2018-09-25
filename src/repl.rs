use std::io;
use std::io::prelude::*;
use std::fs::File;

use super::lexer::Lexer;
use super::object::ObjectKind;
use super::parser::Parser;
use super::evaluator::eval;
use super::environment::{LodeEnvironment, Environment};

pub fn repl(entry_file: Option<&String>) {
    let mut has_entry = false;
    let filename = "./lodestone/lobby.ldst";
    let mut f = File::open(filename).expect("file not found");
    let env = Environment::new();
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    match entry_file {
        Some(entry) => {
            let mut ef = File::open(entry).expect("input error: file not found");
            ef.read_to_string(&mut contents)
                .expect("something went wrong reading the supplied file");
            has_entry = true;
        },
        _ => {}
    }
    let il = Lexer::new(contents);
    let mut ip = Parser::new(il);
    let iprogram = ip.parse_program();
    eval(iprogram, env.clone());
    if !has_entry {
        welcome_message();
        input_loop(env);
    }
}

pub fn input_loop(env: LodeEnvironment) {
    loop {
        print!(">>> ");
        let _= io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect(
            "Failed to read line",
        );
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        let evaluated = eval(program, env.clone());
        match evaluated {
            ObjectKind::Null => {},
            _ => println!("{}", evaluated)
        }
    }
}

pub fn welcome_message() {
    let language_name = "Lodestone";
    println!("Welcome to {}", language_name);
}