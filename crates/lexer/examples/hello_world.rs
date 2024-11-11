use lexer::lex::lex;

fn main() {
    let output = lex("print(\"Hello World\");", "<stdin>").unwrap();
    println!("{output:#?}");
}
