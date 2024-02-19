//! The lexer constants, including type names (`str`, `int`, etc.) and constants (`true`, `false`, `none`).
// I Language lexer constants.
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


////////////////
// TYPE NAMES //
////////////////

/// Type **name** tokens representing type names (`str`, `int`, etc., **NOT** `"my string"` or `1234`) in the lexer.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Type {
    /// The `str` type. Alias: `string`. The type of string literals (e.g. `"my string"`).
    Str,
    /// The `int` type. Alias: `integer`. The type of integer literals (e.g. `12`).
    Int,
    /// The `bool` type. Alias: `boolean`. The type of boolean literals (`true`, `false`).
    Bool,
}

impl GetToken for Type {
    #[inline(always)]
    fn get_token(location: Location, buffer: &Vec<char>) -> Option<Token> {
        let content: &str = &buffer.iter().collect::<String>();

        match content {
            "str" | "string" => Some(Token {
                location,
                content: "string".to_owned(),
                token_type: TokenType::Type(Type::Str),
            }),
            "int" | "integer" => Some(Token {
                location,
                content: "integer".to_owned(),
                token_type: TokenType::Type(Type::Int),
            }),
            "bool" | "boolean" => Some(Token {
                location,
                content: "boolean".to_owned(),
                token_type: TokenType::Type(Type::Bool),
            }),
            _ => None,
        }
    }
}
