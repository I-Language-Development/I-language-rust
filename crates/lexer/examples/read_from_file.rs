use lexer;
use std;

fn main() {
    let file = std::fs::read("examples/hello_world.il").unwrap();
    let input: &str = &String::from_utf8_lossy(&file);

    let output = lexer::lex::lex(input, "hello_world.il").unwrap();
    println!("{output:#?}");
}
