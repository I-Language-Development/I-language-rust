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

/////////////
// IMPORTS //
/////////////

use crate::tokens::constant::Type;
use crate::tokens::keyword::Keyword;
use crate::tokens::token::{GetToken, Location, Token, TokenType, TypeDefinition};

use tools::error;
use tools::iterator::ConditionalPeeking;

use log::trace;


//////////////////
// LEX FUNCTION //
//////////////////

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
    let mut buffer: Vec<char>;
    let mut index: usize;

    for (mut line_number, line) in input.split('\n').enumerate() {
        line_number += 1;

        buffer = vec![];
        iterator = line.chars().enumerate().peekable();

        while let Some((zero_based_index, character)) = iterator.next() {
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

            if character == '"' || character == '\'' {
                TypeDefinition::lex_string(&mut iterator, line, location, character);
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
                if let Some(value) = TokenType::lex_mark(&mut iterator, line, location, character) {
                    result.push(value);
                } else {
                    // Syntax error
                }
            } else if character.is_ascii_digit() {
                buffer.push(character);
                buffer.append(
                    &mut iterator
                        .peek_while(|&(_, next_character)| next_character.is_ascii_digit())
                        .iter()
                        .map(|&(_, found)| found)
                        .collect::<Vec<char>>(),
                );

                result.push(Token {
                    location: location.clone(),
                    content: buffer.iter().collect::<String>(),
                    token_type: TokenType::TypeDefinition(TypeDefinition::Integer),
                });
                buffer.clear();
            } else if character.is_alphabetic() || character == '_' {
                buffer.push(character);
                buffer.append(
                    &mut iterator
                        .peek_while(|&(_, next_character)| {
                            next_character.is_alphabetic() || next_character == '_'
                        })
                        .iter()
                        .map(|&(_, found)| found)
                        .collect::<Vec<char>>(),
                );

                if let Some(value) = Keyword::get_token(location.clone(), &buffer) {
                    result.push(value);
                    buffer.clear();
                } else if let Some(value) = Type::get_token(location.clone(), &buffer) {
                    result.push(value);
                    buffer.clear();
                } else if let Some(value) = TypeDefinition::get_token(location.clone(), &buffer) {
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
