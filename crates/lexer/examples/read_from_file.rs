use lexer::lex::lex;
use std::fs;

fn main() {
    let file = fs::read("examples/hello_world.il").unwrap();
    let input: &str = &String::from_utf8_lossy(&file);

    let output = lex(input, "hello_world.il").unwrap();
    println!("{output:#?}");
}
