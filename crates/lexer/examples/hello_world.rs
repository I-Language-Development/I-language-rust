use lexer;

fn main() {
    let output = lexer::lex::lex("print(\"Hello World\");", "<stdin>");
    println!("{output:#?}");
}
