// I Language lexer marks.
// Version: 1.0.0
//
// Copyright (c) 2023-present I Language Development.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the 'Software'),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
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

use crate::tokens::token::*;


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
    fn get_token(location: Location, buffer: &Vec<char>) -> Option<(Token, usize)> {
        let content: &str = &buffer.iter().collect::<String>();

        match content {
            "+" => Some((
                Token {
                    location,
                    content: "+".to_owned(),
                    token_type: TokenType::Mark(Mark::Add),
                },
                1,
            )),
            "+=" => Some((
                Token {
                    location,
                    content: "+=".to_owned(),
                    token_type: TokenType::Mark(Mark::AddAssign),
                },
                2,
            )),
            "&&" => Some((
                Token {
                    location,
                    content: "&&".to_owned(),
                    token_type: TokenType::Mark(Mark::And),
                },
                2,
            )),
            "->" => Some((
                Token {
                    location,
                    content: "->".to_owned(),
                    token_type: TokenType::Mark(Mark::Arrow),
                },
                2,
            )),
            "=" => Some((
                Token {
                    location,
                    content: "=".to_owned(),
                    token_type: TokenType::Mark(Mark::Assign),
                },
                1,
            )),
            "@" => Some((
                Token {
                    location,
                    content: "@".to_owned(),
                    token_type: TokenType::Mark(Mark::At),
                },
                1,
            )),
            "!" => Some((
                Token {
                    location,
                    content: "!".to_owned(),
                    token_type: TokenType::Mark(Mark::Bang),
                },
                1,
            )),
            "&" => Some((
                Token {
                    location,
                    content: "&".to_owned(),
                    token_type: TokenType::Mark(Mark::BitAnd),
                },
                1,
            )),
            "&=" => Some((
                Token {
                    location,
                    content: "&=".to_owned(),
                    token_type: TokenType::Mark(Mark::BitAndAssign),
                },
                2,
            )),
            "|" => Some((
                Token {
                    location,
                    content: "|".to_owned(),
                    token_type: TokenType::Mark(Mark::BitOr),
                },
                1,
            )),
            "|=" => Some((
                Token {
                    location,
                    content: "|=".to_owned(),
                    token_type: TokenType::Mark(Mark::BitOrAssign),
                },
                2,
            )),
            "^" => Some((
                Token {
                    location,
                    content: "^".to_owned(),
                    token_type: TokenType::Mark(Mark::BitXor),
                },
                1,
            )),
            "^=" => Some((
                Token {
                    location,
                    content: "^=".to_owned(),
                    token_type: TokenType::Mark(Mark::BitXorAssign),
                },
                2,
            )),
            "{" => Some((
                Token {
                    location,
                    content: "{".to_owned(),
                    token_type: TokenType::Mark(Mark::BraceOpen),
                },
                1,
            )),
            "}" => Some((
                Token {
                    location,
                    content: "}".to_owned(),
                    token_type: TokenType::Mark(Mark::BraceClose),
                },
                1,
            )),
            "[" => Some((
                Token {
                    location,
                    content: "[".to_owned(),
                    token_type: TokenType::Mark(Mark::BracketOpen),
                },
                1,
            )),
            "]" => Some((
                Token {
                    location,
                    content: "]".to_owned(),
                    token_type: TokenType::Mark(Mark::BracketClose),
                },
                1,
            )),
            ":" => Some((
                Token {
                    location,
                    content: ":".to_owned(),
                    token_type: TokenType::Mark(Mark::Colon),
                },
                1,
            )),
            "," => Some((
                Token {
                    location,
                    content: ",".to_owned(),
                    token_type: TokenType::Mark(Mark::Comma),
                },
                1,
            )),
            "--" => Some((
                Token {
                    location,
                    content: "--".to_owned(),
                    token_type: TokenType::Mark(Mark::Decrease),
                },
                2,
            )),
            "/" => Some((
                Token {
                    location,
                    content: "/".to_owned(),
                    token_type: TokenType::Mark(Mark::Divide),
                },
                1,
            )),
            "/=" => Some((
                Token {
                    location,
                    content: "/=".to_owned(),
                    token_type: TokenType::Mark(Mark::DivideAssign),
                },
                2,
            )),
            "." => Some((
                Token {
                    location,
                    content: ".".to_owned(),
                    token_type: TokenType::Mark(Mark::Dot),
                },
                1,
            )),
            "==" => Some((
                Token {
                    location,
                    content: "==".to_owned(),
                    token_type: TokenType::Mark(Mark::Equal),
                },
                2,
            )),
            "**" => Some((
                Token {
                    location,
                    content: "**".to_owned(),
                    token_type: TokenType::Mark(Mark::Exponentiation),
                },
                2,
            )),
            ">" => Some((
                Token {
                    location,
                    content: ">".to_owned(),
                    token_type: TokenType::Mark(Mark::Greater),
                },
                1,
            )),
            ">=" => Some((
                Token {
                    location,
                    content: ">=".to_owned(),
                    token_type: TokenType::Mark(Mark::GreaterEqual),
                },
                2,
            )),
            "++" => Some((
                Token {
                    location,
                    content: "++".to_owned(),
                    token_type: TokenType::Mark(Mark::Increase),
                },
                2,
            )),
            "<" => Some((
                Token {
                    location,
                    content: "<".to_owned(),
                    token_type: TokenType::Mark(Mark::Less),
                },
                1,
            )),
            "<=" => Some((
                Token {
                    location,
                    content: "<=".to_owned(),
                    token_type: TokenType::Mark(Mark::LessEqual),
                },
                2,
            )),
            "%" => Some((
                Token {
                    location,
                    content: "%".to_owned(),
                    token_type: TokenType::Mark(Mark::Modulo),
                },
                1,
            )),
            "%=" => Some((
                Token {
                    location,
                    content: "%=".to_owned(),
                    token_type: TokenType::Mark(Mark::ModuloAssign),
                },
                2,
            )),
            "*" => Some((
                Token {
                    location,
                    content: "*".to_owned(),
                    token_type: TokenType::Mark(Mark::Multiply),
                },
                1,
            )),
            "*=" => Some((
                Token {
                    location,
                    content: "*=".to_owned(),
                    token_type: TokenType::Mark(Mark::MultiplyAssign),
                },
                2,
            )),
            "!=" => Some((
                Token {
                    location,
                    content: "!=".to_owned(),
                    token_type: TokenType::Mark(Mark::NotEqual),
                },
                2,
            )),
            "||" => Some((
                Token {
                    location,
                    content: "||".to_owned(),
                    token_type: TokenType::Mark(Mark::Or),
                },
                2,
            )),
            "(" => Some((
                Token {
                    location,
                    content: "(".to_owned(),
                    token_type: TokenType::Mark(Mark::ParenthesisOpen),
                },
                1,
            )),
            ")" => Some((
                Token {
                    location,
                    content: ")".to_owned(),
                    token_type: TokenType::Mark(Mark::ParenthesisClose),
                },
                1,
            )),
            "?" => Some((
                Token {
                    location,
                    content: "?".to_owned(),
                    token_type: TokenType::Mark(Mark::QuestionMark),
                },
                1,
            )),
            ".." => Some((
                Token {
                    location,
                    content: "..".to_owned(),
                    token_type: TokenType::Mark(Mark::Range),
                },
                2,
            )),
            ";" => Some((
                Token {
                    location,
                    content: ";".to_owned(),
                    token_type: TokenType::Mark(Mark::Semicolon),
                },
                1,
            )),
            "<<" => Some((
                Token {
                    location,
                    content: "<<".to_owned(),
                    token_type: TokenType::Mark(Mark::ShiftLeft),
                },
                2,
            )),
            "<<=" => Some((
                Token {
                    location,
                    content: "<<=".to_owned(),
                    token_type: TokenType::Mark(Mark::ShiftLeftAssign),
                },
                3,
            )),
            ">>" => Some((
                Token {
                    location,
                    content: ">>".to_owned(),
                    token_type: TokenType::Mark(Mark::ShiftRight),
                },
                2,
            )),
            ">>=" => Some((
                Token {
                    location,
                    content: ">>=".to_owned(),
                    token_type: TokenType::Mark(Mark::ShiftRightAssign),
                },
                3,
            )),
            "-" => Some((
                Token {
                    location,
                    content: "-".to_owned(),
                    token_type: TokenType::Mark(Mark::Subtract),
                },
                1,
            )),
            "-=" => Some((
                Token {
                    location,
                    content: "-=".to_owned(),
                    token_type: TokenType::Mark(Mark::SubtractAssign),
                },
                2,
            )),
            _ => None,
        }
    }
}
