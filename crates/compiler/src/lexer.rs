// lexer.rs

#[derive(Debug, PartialEq)]
enum Token {
    Number(i64),
    Plus,
    Minus,
    Multiply,
    Divide,
}

fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    // Implement your lexer logic here
    // For simplicity, let's assume input contains only numbers, +, -, *, /
    // You'd need to handle whitespace, error cases, and more complex tokens in a real compiler
    for token_str in input.split_whitespace() {
        match token_str {
            "+" => tokens.push(Token::Plus),
            "-" => tokens.push(Token::Minus),
            "*" => tokens.push(Token::Multiply),
            "/" => tokens.push(Token::Divide),
            _ => {
                if let Ok(num) = token_str.parse::<i64>() {
                    tokens.push(Token::Number(num));
                }
            }
        }
    }
    tokens
}
