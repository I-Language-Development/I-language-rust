use lexer;

fn main() {
    let output = lexer::lex::lex("1 + 1", "<stdin>");
    println!("{output:#?}");
}
