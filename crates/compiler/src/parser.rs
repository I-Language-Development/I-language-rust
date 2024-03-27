// parser.rs

#[derive(Debug)]
enum Expr {
    Number(i64),
    BinaryOp(Box<Expr>, Token, Box<Expr>),
}

fn parse(tokens: &[Token]) -> Expr {
    // Implement your parser logic here
    // For simplicity, assume correct input and only handle + and * operators
    // You'd need to handle operator precedence and other expressions in a real compiler
    // Recursive descent parsing is a common approach
    // Build the AST based on the token stream
    // ...
}

fn main() {
    let input = "10 + 5 * 3";
    let tokens = lex(input);
    let ast = parse(&tokens);
    println!("{:?}", ast);
}
