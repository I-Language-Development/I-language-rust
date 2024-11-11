use lexer::lex::lex;

fn main() {
    let output = lex("1 + 1", "<stdin>").unwrap();
    println!("{output:#?}");
}
