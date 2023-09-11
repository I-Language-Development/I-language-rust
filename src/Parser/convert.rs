// I Language parser converter.
// Version: 0.1.0

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

////////////////////////////////
// IMPORTS AND USE STATEMENTS //
////////////////////////////////

use crate::Parser::parser::Rule;
use crate::Parser::token::*;

use pest::iterators::{Pair, Pairs};


////////////////////
// CONVERT HELPER //
////////////////////

pub fn convert_helper<'a>(pair: Pair<'a, Rule>, token_type: TokenType) -> Token {
    Token {
        token_type: token_type,
        content: String::from(pair.as_str()),
        inner: convert(pair.clone().into_inner()),
        position: Position {
            line: pair.line_col().0,
            colon: pair.line_col().1,
        },
    }
}


/////////////
// CONVERT //
/////////////

pub fn convert<'a>(input: Pairs<'a, Rule>) -> Option<Vec<Token>> {
    let mut result: Vec<Token> = Vec::new();

    for pair in input {
        println!("{}", pair);

        match pair.as_rule() {
            Rule::import => result.push(convert_helper(pair, TokenType::Keyword(Keyword::Import))),
            Rule::import_from => {
                result.push(convert_helper(pair, TokenType::Keyword(Keyword::_From)))
            }
            Rule::import_as => result.push(convert_helper(pair, TokenType::Keyword(Keyword::As))),

            Rule::_use => result.push(convert_helper(pair, TokenType::Keyword(Keyword::Use))),
            Rule::use_feature => {
                result.push(convert_helper(pair, TokenType::Keyword(Keyword::Import)))
            }
            Rule::system => result.push(convert_helper(
                pair,
                TokenType::UseFeature(UseFeature::System),
            )),
            Rule::shell => result.push(convert_helper(
                pair,
                TokenType::UseFeature(UseFeature::Shell),
            )),
            Rule::settings => result.push(convert_helper(
                pair,
                TokenType::UseFeature(UseFeature::Settings),
            )),
            Rule::garbage_collection => result.push(convert_helper(
                pair,
                TokenType::UseFeature(UseFeature::GarbageCollection),
            )),
            Rule::rust => result.push(convert_helper(
                pair,
                TokenType::UseFeature(UseFeature::Rust),
            )),
            Rule::speedups => result.push(convert_helper(
                pair,
                TokenType::UseFeature(UseFeature::Speedups),
            )),

            Rule::assignment => {
                result.push(convert_helper(pair, TokenType::Mark(Mark::Assignment)))
            }
            Rule::_const => result.push(convert_helper(pair, TokenType::Mark(Mark::Constant))),
            Rule::var => result.push(convert_helper(pair, TokenType::Mark(Mark::Variable))),

            Rule::type_definition => {
                result.push(convert_helper(pair, TokenType::Mark(Mark::TypeDefinition)))
            }
            Rule::list => result.push(convert_helper(
                pair,
                TokenType::TypeDefinition(TypeDefinition::List),
            )),
            Rule::string => result.push(convert_helper(
                pair,
                TokenType::TypeDefinition(TypeDefinition::String),
            )),
            Rule::string_content_single_quote | Rule::string_content_double_quote => {
                result.push(convert_helper(
                    pair,
                    TokenType::TypeDefinition(TypeDefinition::StringContent),
                ))
            }
            Rule::multiline_string_content_single_quote
            | Rule::multiline_string_content_double_quote => result.push(convert_helper(
                pair,
                TokenType::TypeDefinition(TypeDefinition::StringContentML),
            )),
            Rule::float => result.push(convert_helper(
                pair,
                TokenType::TypeDefinition(TypeDefinition::Float),
            )),
            Rule::integer_with_sign => result.push(convert_helper(
                pair,
                TokenType::TypeDefinition(TypeDefinition::Integer),
            )),
            Rule::integer_sign => result.push(convert_helper(
                pair,
                TokenType::TypeDefinition(TypeDefinition::IntegerSign),
            )),
            Rule::integer => result.push(convert_helper(
                pair,
                TokenType::TypeDefinition(TypeDefinition::Integer),
            )),

            Rule::identifier => result.push(convert_helper(pair, TokenType::Identifier)),

            Rule::keywords => match pair.as_str() {
                "as" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::As))),
                "break" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::Break))),
                "case" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::Case))),
                "catch" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::Catch))),
                "class" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::Class))),
                "const" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::Constant))),
                "continue" => {
                    result.push(convert_helper(pair, TokenType::Keyword(Keyword::Continue)))
                }
                "default" => {
                    result.push(convert_helper(pair, TokenType::Keyword(Keyword::Default)))
                }
                "delete" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::Delete))),
                "else" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::Else))),
                "elseif" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::ElseIf))),
                "finally" => {
                    result.push(convert_helper(pair, TokenType::Keyword(Keyword::Finally)))
                }
                "for" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::For))),
                "from" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::_From))),
                "func" | "function" => {
                    result.push(convert_helper(pair, TokenType::Keyword(Keyword::Function)))
                }
                "if" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::If))),
                "import" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::Import))),
                "in" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::In))),
                "match" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::Match))),
                "not" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::Not))),
                "raise" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::Raise))),
                "return" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::Return))),
                "try" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::Try))),
                "type" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::Type))),
                "use" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::Use))),
                "var" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::Variable))),
                "while" => result.push(convert_helper(pair, TokenType::Keyword(Keyword::While))),
                _ => {}
            },
            Rule::_type => match pair.as_str() {
                "array" => result.push(convert_helper(
                    pair,
                    TokenType::CompoundType(CompoundType::Array(BaseType::Any)),
                )),
                "bool" => result.push(convert_helper(pair, TokenType::BaseType(BaseType::Boolean))),
                "char" => result.push(convert_helper(pair, TokenType::BaseType(BaseType::Char))),
                "dict" | "dictionary" => result.push(convert_helper(
                    pair,
                    TokenType::CompoundType(CompoundType::HashMap(BaseType::Any, BaseType::Any)),
                )),
                "float" => result.push(convert_helper(pair, TokenType::BaseType(BaseType::Float))),
                "int" | "integer" => {
                    result.push(convert_helper(pair, TokenType::BaseType(BaseType::Integer)))
                }
                "list" => result.push(convert_helper(
                    pair,
                    TokenType::CompoundType(CompoundType::List(BaseType::Any)),
                )),
                "str" | "string" => result.push(convert_helper(
                    pair,
                    TokenType::CompoundType(CompoundType::String),
                )),
                "tuple" => result.push(convert_helper(
                    pair,
                    TokenType::CompoundType(CompoundType::Tuple(BaseType::Any)),
                )),
                _ => {}
            },

            Rule::wsc | Rule::EOI => {}
            _ => println!("Found bad thing"),
        }
    }

    if result.is_empty() {
        return None;
    } else {
        return Some(result);
    }
}
