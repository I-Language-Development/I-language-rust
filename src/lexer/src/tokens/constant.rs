// I Language lexer constants.
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


////////////////////
// TRUE CONSTANTS //
////////////////////

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Constant {
    True,
    False,
}

impl GetToken for Constant {
    fn get_token(location: Location, buffer: &Vec<char>) -> Option<(Token, usize)> {
        let content: &str = &buffer.into_iter().collect::<String>();

        match content {
            "true" => Some((
                Token {
                    location: location,
                    content: "true".to_owned(),
                    token_type: TokenType::TypeDefinition(TypeDefinition::Boolean(Constant::True)),
                },
                4,
            )),
            "false" => Some((
                Token {
                    location: location,
                    content: "false".to_owned(),
                    token_type: TokenType::TypeDefinition(TypeDefinition::Boolean(Constant::False)),
                },
                5,
            )),
            "None" => Some((
                Token {
                    location: location,
                    content: "None".to_owned(),
                    token_type: TokenType::TypeDefinition(TypeDefinition::None),
                },
                4,
            )),
            _ => None,
        }
    }
}


////////////////
// TYPE NAMES //
////////////////

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Type {
    Str,
    Int,
    Boolean,
}

impl GetToken for Type {
    fn get_token(location: Location, buffer: &Vec<char>) -> Option<(Token, usize)> {
        let content: &str = &buffer.into_iter().collect::<String>();

        match content {
            "str" | "string" => Some((
                Token {
                    location: location,
                    content: "string".to_string(),
                    token_type: TokenType::Type(Type::Str),
                },
                content.len(),
            )),
            "int" | "integer" => Some((
                Token {
                    location: location,
                    content: "integer".to_string(),
                    token_type: TokenType::Type(Type::Int),
                },
                content.len(),
            )),
            "bool" | "boolean" => Some((
                Token {
                    location: location,
                    content: "boolean".to_string(),
                    token_type: TokenType::Type(Type::Boolean),
                },
                content.len(),
            )),
            _ => None,
        }
    }
}
