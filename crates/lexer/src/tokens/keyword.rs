// I Language lexer keywords.
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
    Delete, // Replace with "gc.delete"?
    Else,
    Finally,
    For,
    Function, // Replace with "fn" or "func"?
    If,
    Import,
    Match,
    Pub,
    Return,
    Throw,
    Try,
    Use,
    While,
}

// TODO (ElBe): Possible way of doing it: one keyword enum var, like Identifier, determine keyword using content
// TODO (ElBe): Possible way of doing it: file for each token type containing what to match and the token type (grammar file)
//              "break"     KEYWORD(break)
impl GetToken for Keyword {
    #[inline]
    #[allow(clippy::too_many_lines)]
    fn get_token(location: Location, buffer: &Vec<char>) -> Option<Token> {
        let content: &str = &buffer.iter().collect::<String>();

        match content {
            "break" => Some(Token {
                location,
                content: "break".to_owned(),
                token_type: TokenType::Keyword(Keyword::Break),
            }),
            "case" => Some(Token {
                location,
                content: "case".to_owned(),
                token_type: TokenType::Keyword(Keyword::Case),
            }),
            "catch" => Some(Token {
                location,
                content: "catch".to_owned(),
                token_type: TokenType::Keyword(Keyword::Catch),
            }),
            "class" => Some(Token {
                location,
                content: "class".to_owned(),
                token_type: TokenType::Keyword(Keyword::Class),
            }),
            "continue" => Some(Token {
                location,
                content: "continue".to_owned(),
                token_type: TokenType::Keyword(Keyword::Continue),
            }),
            "default" => Some(Token {
                location,
                content: "default".to_owned(),
                token_type: TokenType::Keyword(Keyword::Default),
            }),
            "delete" => Some(Token {
                location,
                content: "delete".to_owned(),
                token_type: TokenType::Keyword(Keyword::Delete),
            }),
            "else" => Some(Token {
                location,
                content: "else".to_owned(),
                token_type: TokenType::Keyword(Keyword::Else),
            }),
            "finally" => Some(Token {
                location,
                content: "finally".to_owned(),
                token_type: TokenType::Keyword(Keyword::Finally),
            }),
            "for" => Some(Token {
                location,
                content: "for".to_owned(),
                token_type: TokenType::Keyword(Keyword::For),
            }),
            "function" => Some(Token {
                location,
                content: "function".to_owned(),
                token_type: TokenType::Keyword(Keyword::Function),
            }),
            "if" => Some(Token {
                location,
                content: "if".to_owned(),
                token_type: TokenType::Keyword(Keyword::If),
            }),
            "import" => Some(Token {
                location,
                content: "import".to_owned(),
                token_type: TokenType::Keyword(Keyword::Import),
            }),
            "match" => Some(Token {
                location,
                content: "match".to_owned(),
                token_type: TokenType::Keyword(Keyword::Match),
            }),
            "pub" => Some(Token {
                location,
                content: "pub".to_owned(),
                token_type: TokenType::Keyword(Keyword::Pub),
            }),
            "return" => Some(Token {
                location,
                content: "return".to_owned(),
                token_type: TokenType::Keyword(Keyword::Return),
            }),
            "throw" => Some(Token {
                location,
                content: "throw".to_owned(),
                token_type: TokenType::Keyword(Keyword::Throw),
            }),
            "try" => Some(Token {
                location,
                content: "try".to_owned(),
                token_type: TokenType::Keyword(Keyword::Try),
            }),
            "use" => Some(Token {
                location,
                content: "use".to_owned(),
                token_type: TokenType::Keyword(Keyword::Use),
            }),
            "while" => Some(Token {
                location,
                content: "while".to_owned(),
                token_type: TokenType::Keyword(Keyword::While),
            }),
            _ => None,
        }
    }
}
