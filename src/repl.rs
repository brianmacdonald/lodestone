use std::io;

pub fn repl() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(
        "Failed to read line",
    );
    println!("You inputted: {}", input);
}
