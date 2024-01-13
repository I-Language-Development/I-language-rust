//! Lexes (tokenizes) a input string into tokens.
// I Language lexer.
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

// Many false positives
#![allow(clippy::pattern_type_mismatch)]

/////////////
// IMPORTS //
/////////////

use crate::tokens::constant::{Constant, Type};
use crate::tokens::keyword::Keyword;
use crate::tokens::mark::Mark;
use crate::tokens::token::{GetToken, Location, Token, TokenType, TypeDefinition};

use tools::error;
use tools::iterator::ConditionalPeeking;

use log::{error, trace};


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
#[inline] // Suggesting inlining due to rare calls to the function
#[allow(clippy::too_many_lines)]
pub fn lex(input: &str, file: &str) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];

    let mut iterator: std::iter::Peekable<std::iter::Enumerate<std::str::Chars>>;
    let mut index: usize;

    let mut buffer: Vec<char>;
    let mut buffer_location: Option<Location> = None;

    let mut skips: usize = 0;

    'outer: for (mut line_number, line) in input.split('\n').enumerate() {
        line_number += 1;

        buffer = vec![];
        iterator = line.chars().enumerate().peekable();

        while let Some((zero_based_index, character)) = iterator.next() {
            if skips > 0 {
                skips -= 1;
                continue;
            }

            if character.is_whitespace() {
                continue;
            }

            index = zero_based_index + 1;
            let location: Location = Location {
                file: file.to_owned(),
                line: line_number,
                column: index,
            };

            trace!("Lexing character {character} in line {line_number}, column {index}.");

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
                        unterminated_string_error();
                    }
                } else {
                    unterminated_string_error();
                }

                result.push(Token {
                    location,
                    content: buffer.iter().collect::<String>(),
                    token_type: TokenType::TypeDefinition(TypeDefinition::Str),
                });
                buffer.clear();
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
                        unterminated_string_error();
                    }
                } else {
                    unterminated_string_error();
                }

                result.push(Token {
                    location,
                    content: buffer.iter().collect::<String>(),
                    token_type: TokenType::TypeDefinition(TypeDefinition::Str),
                });
                buffer.clear();
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

                if let Some(&(_, next_character)) = iterator.clone().peek() {
                    #[allow(clippy::else_if_without_else)]
                    if matches!(
                        next_character,
                        '+' | '-' | '/' | '*' | '=' | '&' | '|' | '.'
                    ) {
                        iterator.next();
                        buffer.push(next_character);
                    /* Three character exceptions ("<<=", ">>=") */
                    } else if matches!(next_character, '<' | '>') {
                        iterator.next();
                        buffer.push(next_character);

                        if let Some(&(_, '=')) = iterator.clone().peek() {
                            iterator.next();
                            buffer.push('=');
                        }
                    }
                }

                #[allow(clippy::else_if_without_else)]
                if &buffer.iter().collect::<String>() == "/*" {
                    let last_character: std::cell::Cell<char> = std::cell::Cell::new('\0');
                    buffer = iterator
                        .peek_until(|&(_, next_character)| {
                            let value: bool = last_character.get() == '*' && next_character == '/';
                            last_character.set(next_character);
                            value
                        })
                        .iter()
                        .map(|(_, found)| *found)
                        .collect::<Vec<char>>();

                    if buffer.last() != Some(&'*') {
                        eprintln!("Unclosed comment...");
                        eprintln!("Proper errors soon...");
                        std::process::exit(1);
                    }

                    iterator.next();

                    // We can be assured the slicing won't panic unless the comment immediately
                    #[allow(clippy::indexing_slicing)]
                    result.push(Token {
                        location,
                        content: buffer[..(buffer.len() - 1)]
                            .iter()
                            .collect::<String>()
                            .trim()
                            .to_owned(),
                        token_type: TokenType::Comment,
                    });
                    buffer.clear();
                } else if &buffer.iter().collect::<String>() == "//" {
                    buffer = line[(index + 1)..].chars().collect::<Vec<char>>();

                    result.push(Token {
                        location,
                        content: buffer.iter().collect::<String>().trim().to_owned(),
                        token_type: TokenType::Comment,
                    });

                    continue 'outer;
                } else if let Some(value) = Mark::get_token(location.clone(), &buffer) {
                    result.push(value);
                    buffer.clear();
                }
            } else if character.is_ascii_digit() {
                buffer.push(character);
                buffer.append(
                    &mut iterator
                        .peek_while(|&(_, next_character)| {
                            next_character.is_ascii_digit()
                                || next_character == '_'
                                || next_character == '.'
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
                        let dots: std::cell::Cell<u8> = std::cell::Cell::new(0);
                        let found_dots: Vec<&char> =
                            buffer.iter().peekable().peek_until(|&&next_character| {
                                if next_character == '.' {
                                    dots.set(dots.get() + 1);
                                } else {
                                    dots.set(0);
                                }

                                dots.get() > 2
                            });
                        if found_dots.len() != buffer.len() {
                            error!(
                                "Invalid float literal at {index}:\n{}\n{}^ = help: to many dots",
                                buffer.iter().collect::<String>(),
                                " ".repeat(found_dots.len())
                            );
                        }
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

                if let Some(value) = Keyword::get_token(location.clone(), &buffer) {
                    result.push(value);
                    buffer.clear();
                } else if let Some(value) = Type::get_token(location.clone(), &buffer) {
                    result.push(value);
                    buffer.clear();
                } else if let Some(value) = Constant::get_token(location.clone(), &buffer) {
                    result.push(value);
                    buffer.clear();
                } else {
                    result.push(Token {
                        location: location.clone(),
                        content: buffer.iter().collect::<String>(),
                        token_type: TokenType::Identifier,
                    });

                    buffer.clear();
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
