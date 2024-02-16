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

use tools::iterator::ConditionalPeeking;

use annotate_snippets;
use log::trace;


//////////////////
// LEX FUNCTION //
//////////////////

/// Lexes (tokenizes) a input string into a vector of [`Token`]s.
///
/// # Parameters
///
/// - `input`: The input string to lex.
/// - `file`: The file name of the input. Can be `<stdin>`.
///
/// # Returns
///
/// A result of a vector of lexed [`Token`]s.
///
/// # Errors
///
/// Errors when there are any kind of syntax errors, that are detectable in the lexer (e.g. `if = false` will be detected in the parser).
///
/// # Examples
///
/// ```rust
/// # use lexer::lex;
/// # use lexer::tokens::mark;
/// # use lexer::tokens::token;
/// assert_eq!(lex::lex("1 + 1", "<stdin>"),
/// Ok(vec![
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
/// ]));
/// ```
///
/// # See also
///
/// - [`Token`]
/// - [`Location`]
#[inline] // Suggesting inlining due to rare calls to the function
#[allow(clippy::too_many_lines)]
// TODO (ElBe, Ranastra): Switch to custom error type
pub fn lex(input: &str, file: &str) -> Result<Vec<Token>, String> {
    let mut error: Option<String> = None;
    let mut result: Vec<Token> = vec![];

    let mut iterator: std::iter::Peekable<std::iter::Enumerate<std::str::Chars>>;
    let mut buffer: Vec<char>;
    let mut index: usize;

    for (mut line_number, line) in input.split('\n').enumerate() {
        line_number += 1;

        buffer = vec![];
        iterator = line.chars().enumerate().peekable();

        while let Some((zero_based_index, character)) = iterator.next() {
            let start: std::time::Instant = std::time::Instant::now();
            if character.is_whitespace() {
                continue;
            }

            index = zero_based_index + 1;
            let location: Location = Location {
                file: file.to_owned(),
                line: line_number,
                column: index,
            };

            if character == '"' || character == '\'' {
                result.push(TypeDefinition::lex_string(
                    &mut iterator,
                    line,
                    location,
                    character,
                ));
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
                    | '~'
                    | '('
                    | ')'
                    | '{'
                    | '}'
                    | '['
                    | ']'
            ) {
                if let Ok(Some(value)) =
                    TokenType::lex_mark(&mut iterator, line, location.clone(), character)
                {
                    result.push(value);
                } else {
                    let snippet: annotate_snippets::Snippet = annotate_snippets::Snippet {
                        title: Some(annotate_snippets::Annotation {
                            id: Some("E0001"),
                            label: Some("Syntax error"),
                            annotation_type: annotate_snippets::AnnotationType::Error,
                        }),
                        footer: vec![],
                        slices: vec![annotate_snippets::Slice {
                            source: line,
                            line_start: line_number,
                            origin: Some(file),
                            annotations: vec![annotate_snippets::SourceAnnotation {
                                range: (index - 1, line.len() - iterator.clone().count()),
                                label: "Invalid mark",
                                annotation_type: annotate_snippets::AnnotationType::Error,
                            }],
                            fold: false,
                        }],
                    };

                    let renderer: annotate_snippets::Renderer =
                        annotate_snippets::Renderer::styled();
                    eprintln!("{}", renderer.render(snippet));
                    error = Some(format!("Syntax error: Invalid mark at {location}"));
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
                let snippet: annotate_snippets::Snippet = annotate_snippets::Snippet {
                    title: Some(annotate_snippets::Annotation {
                        id: Some("E0001"),
                        label: Some("Syntax error"),
                        annotation_type: annotate_snippets::AnnotationType::Error,
                    }),
                    footer: vec![],
                    slices: vec![annotate_snippets::Slice {
                        source: line,
                        line_start: line_number,
                        origin: Some(file),
                        annotations: vec![annotate_snippets::SourceAnnotation {
                            range: (index - 1, index),
                            label: "Unexpected character",
                            annotation_type: annotate_snippets::AnnotationType::Error,
                        }],
                        fold: false,
                    }],
                };

                let renderer: annotate_snippets::Renderer = annotate_snippets::Renderer::styled();
                eprintln!("{}", renderer.render(snippet));
                error = Some(format!("Syntax error: Unexpected character at {location}"));
            }

            trace!(
                "Lexing character {character} in line {line_number}, column {index} took {}ms.",
                start.elapsed().as_millis()
            );
        }
    }

    match error {
        Some(message) => Err(format!("Error during lexing (last): {message}")),
        None => Ok(result),
    }
}
