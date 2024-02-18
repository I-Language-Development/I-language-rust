//! The lexers token related types.
// I Language lexer tokens.
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

use core;
use std;

use crate::tokens::constant::Type;
use crate::tokens::keyword::Keyword;
use crate::tokens::mark::Mark;

use tools::iterator::ConditionalPeeking;

use annotate_snippets;

////////////
// TRAITS //
////////////

/// A trait for implementing an own token type.
/// Used for lexing from a buffer into a token.
#[allow(clippy::module_name_repetitions)]
pub trait GetToken {
    /// Lexes the input from the buffer into a [`Token`].
    ///
    /// # Parameters
    ///
    /// - `location`: The location of the first character of the input. This will be included in the result.
    /// - `buffer`: The input buffer to read from.
    ///
    /// # Returns
    ///
    /// A token representing the lexed input, including the given location.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    ///
    /// # use lexer::token::tokens::{GetToken, Location, Token, TokenType};
    ///
    /// enum MyKeyword {
    ///     Foo,
    /// };
    ///
    /// impl GetToken for MyKeyword {
    ///     fn get_token(location: Location, buffer: &Vec<char>) -> Option<Token> {
    ///         if *buffer == vec!['f', 'o', 'o'] {
    ///             Some(Token {
    ///                 location,
    ///                 content: "foo".to_owned(),
    ///                 token_type: TokenType::MyKeyword(MyKeyword::Foo),
    ///             })
    ///         }
    ///
    ///         None
    ///     }
    /// }
    ///
    /// ```
    ///
    /// # See also
    ///
    /// - [`Token`]
    /// - [`Location`]
    #[allow(clippy::ptr_arg)]
    fn get_token(location: Location, buffer: &Vec<char>) -> Option<Token>;
}


////////////////////////////////
// TYPE DEFINITION TOKEN TYPE //
////////////////////////////////

/// Literal tokens representing a literal (`1`, `true`, etc.) in the lexer.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TypeDefinition {
    /// The string literal. Examples: `'"'`, `"hello"`, `'world'`, `"\""`.
    String,
    /// The integer literal. Examples: `1`, `123`, `-10`, `1_000_000`
    Integer,
    /// The `true` literal.
    True,
    /// The `false` literal.
    False,
    /// The none type. Also referred to as null type. The only value is `none`.
    None,
}

impl GetToken for TypeDefinition {
    #[inline(always)]
    fn get_token(location: Location, buffer: &Vec<char>) -> Option<Token> {
        let content: &str = &buffer.iter().collect::<String>();

        match content {
            "true" => Some(Token {
                location,
                content: "true".to_owned(),
                token_type: TokenType::TypeDefinition(TypeDefinition::True),
            }),
            "false" => Some(Token {
                location,
                content: "false".to_owned(),
                token_type: TokenType::TypeDefinition(TypeDefinition::False),
            }),
            "none" => Some(Token {
                location,
                content: "none".to_owned(),
                token_type: TokenType::TypeDefinition(TypeDefinition::None),
            }),
            _ => None,
        }
    }
}

impl TypeDefinition {
    /// Lexes a string into a token of that string.
    ///
    /// # Parameters
    ///
    /// - `iterator`: The current iterator over the characters of the current line. The first appearance of the quote should not be in the iterator, otherwise the function will return an empty string.
    /// - `line`: The non-mutated line.
    /// - `location`: The location of the start of the token.
    /// - `quote_type`: The type of quote used to start the string.
    ///
    /// # Returns
    ///
    /// The string as a [`Token`].
    ///
    /// # Errors
    ///
    /// Errors when the string is not closed.
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// # use std;
    /// # use lexer::tokens::token::{Location, Token, TokenType, TypeDefinition};
    /// let input: &str = "my string'"; // For lexing, the first quote has to be removed
    /// let mut iterator: std::iter::Peekable<std::iter::Enumerate<std::str::Chars>> = input.chars().enumerate().peekable();
    /// # let location: Location = Location {
    /// #    file: "<stdin>".to_owned(),
    /// #    line: 1,
    /// #    column: 1,
    /// # };
    /// assert_eq!(TypeDefinition::lex_string(&mut iterator, input, location.clone(), '\''), Token {
    ///     location,
    ///     content: "my string".to_owned(),
    ///     token_type: TokenType::TypeDefinition(TypeDefinition::String)
    /// });
    ///
    ///
    /// ```
    ///
    /// # See also
    ///
    /// - [`Token`]
    /// - [`TypeDefinition`]
    /// - [`TypeDefinition::String`]
    #[inline(always)]
    // TODO: Errors
    pub fn lex_string(
        iterator: &mut std::iter::Peekable<std::iter::Enumerate<std::str::Chars>>,
        line: &str,
        location: Location,
        quote_type: char,
    ) -> Token {
        let last_character: core::cell::Cell<char> = core::cell::Cell::new('\0');
        let buffer: Vec<char> = iterator
            .peek_while(|&(_, next_character)| {
                let value: bool = last_character.get() != '\\' && next_character == quote_type;
                last_character.set(next_character);
                !value
            })
            .iter()
            .map(|&(_, found)| found)
            .collect::<Vec<char>>();

        let help: String = format!("Add `{quote_type}` here");
        let snippet: annotate_snippets::Snippet = annotate_snippets::Snippet {
            title: Some(annotate_snippets::Annotation {
                id: Some("E0002"),
                label: Some("Unterminated string literal"),
                annotation_type: annotate_snippets::AnnotationType::Error,
            }),
            footer: if buffer.last() == Some(&'\\') {
                vec![annotate_snippets::Annotation {
                    id: None,
                    label: Some("Good help"),
                    annotation_type: annotate_snippets::AnnotationType::Help,
                }]
            } else {
                vec![]
            },
            slices: vec![annotate_snippets::Slice {
                source: line,
                line_start: location.line,
                origin: Some(&location.file),
                annotations: vec![
                    annotate_snippets::SourceAnnotation {
                        range: (location.column - 1, location.column),
                        label: "String started here",
                        annotation_type: annotate_snippets::AnnotationType::Help,
                    },
                    annotate_snippets::SourceAnnotation {
                        range: (
                            location.column + buffer.len() - 1,
                            location.column + buffer.len(),
                        ),
                        label: &help,
                        annotation_type: annotate_snippets::AnnotationType::Help,
                    },
                ],
                fold: false,
            }],
        };

        if let Some((_, next_character)) = iterator.next() {
            if next_character != quote_type {
                let renderer: annotate_snippets::Renderer = annotate_snippets::Renderer::styled();
                eprintln!("{}", renderer.render(snippet));
            }
        } else {
            let renderer: annotate_snippets::Renderer = annotate_snippets::Renderer::styled();
            eprintln!("{}", renderer.render(snippet));
        }

        Token {
            location,
            content: buffer.iter().collect::<String>(),
            token_type: TokenType::TypeDefinition(TypeDefinition::String),
        }
    }
}


//////////////////
// TOKEN STRUCT //
//////////////////

/// Types of tokens in the lexer.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TokenType {
    /// A token representing a type **name**, e.g. `str`.
    Type(Type),
    /// A token representing a literal, e.g. `1`.
    TypeDefinition(TypeDefinition),
    /// A token representing a keyword, e.g. `break`.
    Keyword(Keyword),
    /// A token representing a mark, e.g. `==`.
    Mark(Mark),
    /// A token representing an identifier, e.g. `my_var`.
    Identifier,
    /// A token representing a comment, e.g. `// comment`.
    Comment,
}

impl TokenType {
    /// Lexes a mark into a token of that mark.
    ///
    /// # Parameters
    ///
    /// - `iterator`: The current iterator over the characters of the current line.
    /// - `line`: The non-mutated line.
    /// - `location`: The location of the start of the token.
    /// - `character`: The character the iterator was at when this function was called.
    ///
    /// # Returns
    ///
    /// A token of the lexed mark. Can also be a comment as comments are using characters that marks are using too.
    ///
    /// # Errors
    ///
    /// Errors when no valid mark could be found with that character combination.
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// # use std;
    /// # use lexer::tokens::mark::Mark;
    /// # use lexer::tokens::token::{Location, Token, TokenType};
    /// let input: &str = "==";
    /// let mut iterator: std::iter::Peekable<std::iter::Enumerate<std::str::Chars>> = input.chars().enumerate().peekable();
    /// # let location: Location = Location {
    /// #    file: "<stdin>".to_owned(),
    /// #    line: 1,
    /// #    column: 1,
    /// # };
    /// assert_eq!(TokenType::lex_mark(&mut iterator, input, location.clone(), '='), Ok(Some(Token {
    ///     location,
    ///     content: "==".to_owned(),
    ///     token_type: TokenType::Mark(Mark::Equal)
    /// })));
    ///
    ///
    /// ```
    ///
    /// # See also
    ///
    /// - [`Token`]
    /// - [`TokenType`]
    /// - [`Mark`]
    #[inline(always)]
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::string_slice)]
    pub fn lex_mark(
        iterator: &mut std::iter::Peekable<std::iter::Enumerate<std::str::Chars>>,
        line: &str,
        location: Location,
        character: char,
    ) -> Result<Option<Token>, String> {
        let mut buffer: Vec<char> = vec![character];

        if let Some(&(_, next_character)) = iterator.clone().peek() {
            #[allow(clippy::else_if_without_else)]
            if matches!(
                next_character,
                '+' | '-' | '/' | '*' | '=' | '&' | '|' | '.'
            ) {
                iterator.next();
                buffer.push(next_character);
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
            let last_character: core::cell::Cell<char> = core::cell::Cell::new('\0');
            buffer = iterator
                .peek_until(|&(_, next_character)| {
                    let value: bool = last_character.get() == '*' && next_character == '/';
                    last_character.set(next_character);
                    value
                })
                .iter()
                .map(|&(_, found)| found)
                .collect::<Vec<char>>();

            if buffer.last() != Some(&'*') {
                let snippet: annotate_snippets::Snippet = annotate_snippets::Snippet {
                    title: Some(annotate_snippets::Annotation {
                        id: Some("E0001"),
                        label: Some("Syntax error"),
                        annotation_type: annotate_snippets::AnnotationType::Error,
                    }),
                    footer: vec![],
                    slices: vec![annotate_snippets::Slice {
                        source: line,
                        line_start: location.line,
                        origin: Some(&location.file),
                        annotations: vec![annotate_snippets::SourceAnnotation {
                            range: (location.column - 1, line.len() - iterator.clone().count()),
                            label: "Unterminated comment",
                            annotation_type: annotate_snippets::AnnotationType::Error,
                        }],
                        fold: false,
                    }],
                };

                let renderer: annotate_snippets::Renderer = annotate_snippets::Renderer::styled();
                eprintln!("{}", renderer.render(snippet));
                return Err(format!("Syntax error: Unterminated comment at {location}"));
            }

            iterator.next();

            return Ok(Some(Token {
                location: location.clone(),
                content: buffer[..buffer.len() - 1]
                    .iter()
                    .collect::<String>()
                    .trim()
                    .to_owned(),
                token_type: TokenType::Comment,
            }));
        } else if &buffer.iter().collect::<String>() == "//" {
            buffer = line[location.column + 1..].chars().collect::<Vec<char>>();
            iterator.peek_while(|_| true);

            return Ok(Some(Token {
                location,
                content: buffer.iter().collect::<String>().trim().to_owned(),
                token_type: TokenType::Comment,
            }));
        }

        Ok(Mark::get_token(location.clone(), &buffer))
    }
}

/// The start location of a token in the lexer.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Location {
    /// The file the token is from.
    pub file: String,
    /// The line the token started.
    pub line: usize,
    /// The column the token started.
    pub column: usize,
}

impl core::fmt::Display for Location {
    #[inline]
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "{}:{}:{}", self.file, self.line, self.column)
    }
}

/// A token in the lexer. This is the output of the [`lex`][`crate::lex::lex`] function.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Token {
    /// The location the token started at.
    pub location: Location,
    /// The content of the token. This should not be used for matching tokens, use `token_type` instead.
    pub content: String,
    /// The type of the token. This should be used for matching tokens.
    pub token_type: TokenType,
}
