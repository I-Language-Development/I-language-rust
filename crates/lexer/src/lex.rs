//! Lexes (tokenizes) a input string into tokens.
// I Language lexer.
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

use crate::tokens::constant::*;
use crate::tokens::keyword::*;
use crate::tokens::mark::*;
use crate::tokens::token::*;

use tools::error;
use tools::iterator::ConditionalPeeking;


//////////////////
// LEX FUNCTION //
//////////////////

// TODO (ElBe): Examples and tests
/// Lexes (tokenizes) a input string into a vector of [`Token`]s.
///
/// # Parameters
///
/// - `input`: The input string to lex.
/// - `file`: The file name to show in errors and include in locations of tokens. Can be `<stdin>`.
///
/// # Returns
///
/// A vector of lexed [`Token`]s.
///
/// # Examples
///
/// ```rust
/// # use lexer::lex;
/// # use lexer::tokens::mark;
/// # use lexer::tokens::token;
/// assert_eq!(lex::lex("1 + 1", "<stdin>"),
/// [
///     token::Token {
///         location: token::Location {
///             file: "<stdin>".to_owned(),
///             line: 1,
///             column: 1,
///         },
///         content: "1".to_owned(),
///         token_type: token::TokenType::TypeDefinition(
///             token::TypeDefinition::Integer,
///         ),
///     },
///     token::Token {
///         location: token::Location {
///             file: "<stdin>".to_owned(),
///             line: 1,
///             column: 3,
///         },
///         content: "+".to_owned(),
///         token_type: token::TokenType::Mark(
///             mark::Mark::Add,
///         ),
///     },
///     token::Token {
///         location: token::Location {
///             file: "<stdin>".to_owned(),
///             line: 1,
///             column: 5,
///         },
///         content: "1".to_owned(),
///         token_type: token::TokenType::TypeDefinition(
///             token::TypeDefinition::Integer,
///         ),
///     },
/// ]);
/// ```
///
/// # See also
///
/// - [`Token`]
/// - [`Location`]
pub fn lex(input: &str, file: &str) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];

    let mut chars: std::str::Chars;
    let mut iterator: std::iter::Peekable<std::iter::Enumerate<std::str::Chars>>;
    let mut index: usize;

    let mut buffer: Vec<char>;
    let mut buffer_location: Option<Location> = None;

    let mut skips: usize = 0;

    for (mut line_number, line) in input.split('\n').enumerate() {
        line_number += 1;

        chars = line.chars();
        buffer = vec![];

        iterator = chars.enumerate().peekable();

        while let Some((_index, character)) = iterator.next() {
            if skips > 0 {
                skips -= 1;
                continue;
            }

            index = _index + 1;
            let location: Location = Location {
                file: file.to_string(),
                line: line_number,
                column: index,
            };

            // TODO (ElBe): Add comments (/* */, //)
            if character == '"' {
                let last_character: std::cell::Cell<char> = std::cell::Cell::new('\0');
                buffer = iterator
                    .peek_while(|&(_, next_character)| {
                        let value: bool = last_character.get() != '\\' && next_character == '"';
                        last_character.set(next_character);
                        !value
                    })
                    .iter()
                    .map(|(_, found)| *found)
                    .collect::<Vec<char>>();

                let unterminated_string_error = || {
                    let source: &str = &format!("   {line_number} | {line}");

                    let underline: &str = &format!(
                        "{0}^",
                        " ".repeat(line_number.to_string().len() + index + buffer.len() +  6 /* < Amount of extra characters, in this case the pipe symbol and the five spaces*/)
                    );

                    error::Error::new(
                        "Error",
                        &format!("Unterminated string literal\n --> {location}"),
                        1,
                    )
                    .raise(&format!("{source}\n{underline} = help: Add a `\"` here"));
                };

                if let Some((_, next_character)) = iterator.next() {
                    if next_character != '"' {
                        unterminated_string_error()
                    }
                } else {
                    unterminated_string_error()
                }

                result.push(Token {
                    location,
                    content: buffer.iter().collect::<String>(),
                    token_type: TokenType::TypeDefinition(TypeDefinition::Str),
                });
                buffer.clear()
            } else if character == '\'' {
                let last_character: std::cell::Cell<char> = std::cell::Cell::new('\0');
                buffer = iterator
                    .peek_while(|&(_, next_character)| {
                        let value: bool = last_character.get() != '\\' && next_character == '\'';
                        last_character.set(next_character);
                        !value
                    })
                    .iter()
                    .map(|(_, found)| *found)
                    .collect::<Vec<char>>();

                let unterminated_string_error = || {
                    let source: &str = &format!("   {line_number} | {line}");

                    let underline: &str = &format!(
                        "{0}^",
                        " ".repeat(line_number.to_string().len() + index + buffer.len() +  6 /* < Amount of extra characters, in this case the pipe symbol and the five spaces*/)
                    );

                    error::Error::new(
                        "Error",
                        &format!("Unterminated string literal\n --> {location}"),
                        1,
                    )
                    .raise(&format!("{source}\n{underline} = help: Add a `'` here"));
                };

                if let Some((_, next_character)) = iterator.next() {
                    if next_character != '\'' {
                        unterminated_string_error()
                    }
                } else {
                    unterminated_string_error()
                }

                result.push(Token {
                    location,
                    content: buffer.iter().collect::<String>(),
                    token_type: TokenType::TypeDefinition(TypeDefinition::Str),
                });
                buffer.clear()
            } else if matches!(
                character,
                '+' | '-'
                    | '*'
                    | '/'
                    | '^'
                    | '%'
                    | '@'
                    | '<'
                    | '>'
                    | '!'
                    | '='
                    | '&'
                    | '|'
                    | ':'
                    | '.'
                    | ','
                    | ';'
                    | '('
                    | ')'
                    | '{'
                    | '}'
                    | '['
                    | ']'
            ) {
                buffer.push(character);

                if let Some((_, next_character)) = iterator.clone().peek() {
                    if matches!(
                        *next_character,
                        '+' | '-' | '>' | '*' | '=' | '&' | '|' | '.'
                    ) {
                        iterator.next();
                        buffer.push(*next_character);
                    /* Three character exceptions ("<<=", ">>=") */
                    } else if matches!(*next_character, '<' | '>') {
                        iterator.next();
                        buffer.push(*next_character);

                        if let Some((_, next_character)) = iterator.clone().peek() {
                            if next_character == &'=' {
                                iterator.next();
                                buffer.push('=');
                            }
                        }
                    }
                }

                if let Some((value, _skips)) = Mark::get_token(location.clone(), &buffer) {
                    result.push(value);
                    buffer.clear();
                }
            } else if character.is_ascii_digit() {
                buffer.push(character);
                buffer.append(
                    &mut iterator
                        .peek_while(|&(_, next_character)| {
                            next_character.is_ascii_digit() || next_character == '.'
                        })
                        .iter()
                        .map(|(_, found)| *found)
                        .collect::<Vec<char>>(),
                );

                if buffer.contains(&'.') {
                    if buffer
                        .iter()
                        .filter(|filter_character| **filter_character == '.')
                        .count()
                        > 1
                    {
                        // TODO (ElBe): Proper errors, filter ranges
                        eprintln!("Float with more than one dot");
                    }

                    result.push(Token {
                        location: location.clone(),
                        content: buffer.iter().collect::<String>(),
                        token_type: TokenType::TypeDefinition(TypeDefinition::Float),
                    });
                    buffer.clear();
                } else {
                    result.push(Token {
                        location: location.clone(),
                        content: buffer.iter().collect::<String>(),
                        token_type: TokenType::TypeDefinition(TypeDefinition::Integer),
                    });
                    buffer.clear();
                }
            } else if character.is_alphabetic() || character == '_' {
                buffer.push(character);
                buffer.append(
                    &mut iterator
                        .peek_while(|&(_, next_character)| {
                            next_character.is_alphabetic() || next_character == '_'
                        })
                        .iter()
                        .map(|(_, found)| *found)
                        .collect::<Vec<char>>(),
                );

                if buffer_location.is_none() {
                    buffer_location = Some(location.clone());
                }

                if let Some((value, _skips)) = Keyword::get_token(location.clone(), &buffer) {
                    result.push(value);
                    buffer.clear();
                } else if let Some((value, _skips)) = Type::get_token(location.clone(), &buffer) {
                    result.push(value);
                    buffer.clear();
                } else if let Some((value, _skips)) = Constant::get_token(location.clone(), &buffer)
                {
                    result.push(value);
                    buffer.clear();
                } else {
                    result.push(Token {
                        location: location.clone(),
                        content: buffer.iter().collect::<String>(),
                        token_type: TokenType::Identifier,
                    });

                    buffer.clear()
                }
            } else {
                let source: &str = &format!("   {line_number} | {line}");
                let underline: &str = &format!(
                    "{0:>1$}",
                    '^',
                    line_number.to_string().len() + index + 6 /* < Amount of extra characters, in this case the pipe symbol and the five spaces*/
                );
                error::Error::new(
                    "Error",
                    &format!("Syntax error\n --> {file}:{line_number}:{index}"),
                    2,
                )
                .raise(&format!(
                    "{source}\n{underline} = help: Unexpected character `{character}`"
                ));
            }
        }
    }

    result
}
