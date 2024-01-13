// I Language lexer marks.
// Version: 1.0.0

// Copyright (c) 2023-present I Language Development.

// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the 'Software'),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

/////////////
// IMPORTS //
/////////////

use crate::tokens::token::{GetToken, Location, Token, TokenType};


//////////
// MARK //
//////////

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Mark {
    Add,
    AddAssign,
    And,
    Arrow,
    Assign,
    At,
    Bang,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    Colon,
    Comma,
    Decrease,
    Divide,
    DivideAssign,
    Dot,
    Equal,
    Exponentiation,
    Greater,
    GreaterEqual,
    Increase,
    Less,
    LessEqual,
    Modulo,
    ModuloAssign,
    Multiply,
    MultiplyAssign,
    NotEqual,
    Or,
    ParenthesisOpen,
    ParenthesisClose,
    QuestionMark,
    Range,
    Semicolon,
    ShiftLeft,
    ShiftLeftAssign,
    ShiftRight,
    ShiftRightAssign,
    Subtract,
    SubtractAssign,
}

impl GetToken for Mark {
    #[inline]
    #[allow(clippy::too_many_lines)]
    fn get_token(location: Location, buffer: &Vec<char>) -> Option<Token> {
        let content: &str = &buffer.iter().collect::<String>();

        match content {
            "+" => Some(Token {
                location,
                content: "+".to_owned(),
                token_type: TokenType::Mark(Mark::Add),
            }),
            "+=" => Some(Token {
                location,
                content: "+=".to_owned(),
                token_type: TokenType::Mark(Mark::AddAssign),
            }),
            "&&" => Some(Token {
                location,
                content: "&&".to_owned(),
                token_type: TokenType::Mark(Mark::And),
            }),
            "->" => Some(Token {
                location,
                content: "->".to_owned(),
                token_type: TokenType::Mark(Mark::Arrow),
            }),
            "=" => Some(Token {
                location,
                content: "=".to_owned(),
                token_type: TokenType::Mark(Mark::Assign),
            }),
            "@" => Some(Token {
                location,
                content: "@".to_owned(),
                token_type: TokenType::Mark(Mark::At),
            }),
            "!" => Some(Token {
                location,
                content: "!".to_owned(),
                token_type: TokenType::Mark(Mark::Bang),
            }),
            "&" => Some(Token {
                location,
                content: "&".to_owned(),
                token_type: TokenType::Mark(Mark::BitAnd),
            }),
            "&=" => Some(Token {
                location,
                content: "&=".to_owned(),
                token_type: TokenType::Mark(Mark::BitAndAssign),
            }),
            "|" => Some(Token {
                location,
                content: "|".to_owned(),
                token_type: TokenType::Mark(Mark::BitOr),
            }),
            "|=" => Some(Token {
                location,
                content: "|=".to_owned(),
                token_type: TokenType::Mark(Mark::BitOrAssign),
            }),
            "^" => Some(Token {
                location,
                content: "^".to_owned(),
                token_type: TokenType::Mark(Mark::BitXor),
            }),
            "^=" => Some(Token {
                location,
                content: "^=".to_owned(),
                token_type: TokenType::Mark(Mark::BitXorAssign),
            }),
            "{" => Some(Token {
                location,
                content: "{".to_owned(),
                token_type: TokenType::Mark(Mark::BraceOpen),
            }),
            "}" => Some(Token {
                location,
                content: "}".to_owned(),
                token_type: TokenType::Mark(Mark::BraceClose),
            }),
            "[" => Some(Token {
                location,
                content: "[".to_owned(),
                token_type: TokenType::Mark(Mark::BracketOpen),
            }),
            "]" => Some(Token {
                location,
                content: "]".to_owned(),
                token_type: TokenType::Mark(Mark::BracketClose),
            }),
            ":" => Some(Token {
                location,
                content: ":".to_owned(),
                token_type: TokenType::Mark(Mark::Colon),
            }),
            "," => Some(Token {
                location,
                content: ",".to_owned(),
                token_type: TokenType::Mark(Mark::Comma),
            }),
            "--" => Some(Token {
                location,
                content: "--".to_owned(),
                token_type: TokenType::Mark(Mark::Decrease),
            }),
            "/" => Some(Token {
                location,
                content: "/".to_owned(),
                token_type: TokenType::Mark(Mark::Divide),
            }),
            "/=" => Some(Token {
                location,
                content: "/=".to_owned(),
                token_type: TokenType::Mark(Mark::DivideAssign),
            }),
            "." => Some(Token {
                location,
                content: ".".to_owned(),
                token_type: TokenType::Mark(Mark::Dot),
            }),
            "==" => Some(Token {
                location,
                content: "==".to_owned(),
                token_type: TokenType::Mark(Mark::Equal),
            }),
            "**" => Some(Token {
                location,
                content: "**".to_owned(),
                token_type: TokenType::Mark(Mark::Exponentiation),
            }),
            ">" => Some(Token {
                location,
                content: ">".to_owned(),
                token_type: TokenType::Mark(Mark::Greater),
            }),
            ">=" => Some(Token {
                location,
                content: ">=".to_owned(),
                token_type: TokenType::Mark(Mark::GreaterEqual),
            }),
            "++" => Some(Token {
                location,
                content: "++".to_owned(),
                token_type: TokenType::Mark(Mark::Increase),
            }),
            "<" => Some(Token {
                location,
                content: "<".to_owned(),
                token_type: TokenType::Mark(Mark::Less),
            }),
            "<=" => Some(Token {
                location,
                content: "<=".to_owned(),
                token_type: TokenType::Mark(Mark::LessEqual),
            }),
            "%" => Some(Token {
                location,
                content: "%".to_owned(),
                token_type: TokenType::Mark(Mark::Modulo),
            }),
            "%=" => Some(Token {
                location,
                content: "%=".to_owned(),
                token_type: TokenType::Mark(Mark::ModuloAssign),
            }),
            "*" => Some(Token {
                location,
                content: "*".to_owned(),
                token_type: TokenType::Mark(Mark::Multiply),
            }),
            "*=" => Some(Token {
                location,
                content: "*=".to_owned(),
                token_type: TokenType::Mark(Mark::MultiplyAssign),
            }),
            "!=" => Some(Token {
                location,
                content: "!=".to_owned(),
                token_type: TokenType::Mark(Mark::NotEqual),
            }),
            "||" => Some(Token {
                location,
                content: "||".to_owned(),
                token_type: TokenType::Mark(Mark::Or),
            }),
            "(" => Some(Token {
                location,
                content: "(".to_owned(),
                token_type: TokenType::Mark(Mark::ParenthesisOpen),
            }),
            ")" => Some(Token {
                location,
                content: ")".to_owned(),
                token_type: TokenType::Mark(Mark::ParenthesisClose),
            }),
            "?" => Some(Token {
                location,
                content: "?".to_owned(),
                token_type: TokenType::Mark(Mark::QuestionMark),
            }),
            ".." => Some(Token {
                location,
                content: "..".to_owned(),
                token_type: TokenType::Mark(Mark::Range),
            }),
            ";" => Some(Token {
                location,
                content: ";".to_owned(),
                token_type: TokenType::Mark(Mark::Semicolon),
            }),
            "<<" => Some(Token {
                location,
                content: "<<".to_owned(),
                token_type: TokenType::Mark(Mark::ShiftLeft),
            }),
            "<<=" => Some(Token {
                location,
                content: "<<=".to_owned(),
                token_type: TokenType::Mark(Mark::ShiftLeftAssign),
            }),
            ">>" => Some(Token {
                location,
                content: ">>".to_owned(),
                token_type: TokenType::Mark(Mark::ShiftRight),
            }),
            ">>=" => Some(Token {
                location,
                content: ">>=".to_owned(),
                token_type: TokenType::Mark(Mark::ShiftRightAssign),
            }),
            "-" => Some(Token {
                location,
                content: "-".to_owned(),
                token_type: TokenType::Mark(Mark::Subtract),
            }),
            "-=" => Some(Token {
                location,
                content: "-=".to_owned(),
                token_type: TokenType::Mark(Mark::SubtractAssign),
            }),
            _ => None,
        }
    }
}
