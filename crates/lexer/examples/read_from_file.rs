use lexer;
use std;

fn main() {
    let binding = std::fs::read("examples/hello_world.il").unwrap();
    let input: &str = &String::from_utf8_lossy(&binding);

    let output = lexer::lex::lex(input, "hello_world.il");
    println!("{output:#?}");
}
