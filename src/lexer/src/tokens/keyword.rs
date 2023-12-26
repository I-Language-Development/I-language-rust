// I Language lexer keywords.
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


//////////////
// KEYWORDS //
//////////////

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Keyword {
    Break,
    Case,
    Catch,
    Class,
    Continue,
    Default,
    Delete,
    Else,
    Finally,
    For,
    Function,
    If,
    Import,
    Match,
    Return,
    Throw,
    Try,
    Use,
    While,
}

impl GetToken for Keyword {
    fn get_token(location: Location, buffer: &Vec<char>) -> Option<(Token, usize)> {
        let content: &str = &buffer.iter().collect::<String>();

        match content {
            "break" => Some((
                Token {
                    location,
                    content: "break".to_owned(),
                    token_type: TokenType::Keyword(Keyword::Break),
                },
                5,
            )),
            "case" => Some((
                Token {
                    location,
                    content: "case".to_owned(),
                    token_type: TokenType::Keyword(Keyword::Case),
                },
                4,
            )),
            "catch" => Some((
                Token {
                    location,
                    content: "catch".to_owned(),
                    token_type: TokenType::Keyword(Keyword::Catch),
                },
                5,
            )),
            "class" => Some((
                Token {
                    location,
                    content: "class".to_owned(),
                    token_type: TokenType::Keyword(Keyword::Class),
                },
                5,
            )),
            "continue" => Some((
                Token {
                    location,
                    content: "continue".to_owned(),
                    token_type: TokenType::Keyword(Keyword::Continue),
                },
                8,
            )),
            "default" => Some((
                Token {
                    location,
                    content: "default".to_owned(),
                    token_type: TokenType::Keyword(Keyword::Default),
                },
                7,
            )),
            "delete" => Some((
                Token {
                    location,
                    content: "delete".to_owned(),
                    token_type: TokenType::Keyword(Keyword::Delete),
                },
                6,
            )),
            "else" => Some((
                Token {
                    location,
                    content: "else".to_owned(),
                    token_type: TokenType::Keyword(Keyword::Else),
                },
                4,
            )),
            "finally" => Some((
                Token {
                    location,
                    content: "finally".to_owned(),
                    token_type: TokenType::Keyword(Keyword::Finally),
                },
                6,
            )),
            "for" => Some((
                Token {
                    location,
                    content: "for".to_owned(),
                    token_type: TokenType::Keyword(Keyword::For),
                },
                3,
            )),
            "function" => Some((
                Token {
                    location,
                    content: "function".to_owned(),
                    token_type: TokenType::Keyword(Keyword::Function),
                },
                8,
            )),
            "if" => Some((
                Token {
                    location,
                    content: "if".to_owned(),
                    token_type: TokenType::Keyword(Keyword::If),
                },
                2,
            )),
            "import" => Some((
                Token {
                    location,
                    content: "import".to_owned(),
                    token_type: TokenType::Keyword(Keyword::Import),
                },
                6,
            )),
            "match" => Some((
                Token {
                    location,
                    content: "match".to_owned(),
                    token_type: TokenType::Keyword(Keyword::Match),
                },
                5,
            )),
            "return" => Some((
                Token {
                    location,
                    content: "return".to_owned(),
                    token_type: TokenType::Keyword(Keyword::Return),
                },
                6,
            )),
            "throw" => Some((
                Token {
                    location,
                    content: "throw".to_owned(),
                    token_type: TokenType::Keyword(Keyword::Throw),
                },
                5,
            )),
            "try" => Some((
                Token {
                    location,
                    content: "try".to_owned(),
                    token_type: TokenType::Keyword(Keyword::Try),
                },
                3,
            )),
            "use" => Some((
                Token {
                    location,
                    content: "use".to_owned(),
                    token_type: TokenType::Keyword(Keyword::Use),
                },
                3,
            )),
            "while" => Some((
                Token {
                    location,
                    content: "while".to_owned(),
                    token_type: TokenType::Keyword(Keyword::While),
                },
                5,
            )),
            _ => None,
        }
    }
}
