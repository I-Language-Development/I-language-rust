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

use crate::error::LexerError;
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
    /// }
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


////////////////////////
// LITERAL TOKEN TYPE //
////////////////////////

/// Tokens representing a literal (`1`, `true`, etc.) in the lexer.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Literal {
    /// The string literal. Examples: `'"'`, `"hello"`, `'world'`, `"\""`.
    String,
    /// The integer literal. Examples: `1`, `123`, `-10`, `1_000_000`
    Integer,
    /// The `true` literal.
    True,
    /// The `false` literal.
    False,
    /// The none type. Also referred to as null type.
    None,
}

impl core::fmt::Display for Literal {
    #[inline]
    #[allow(clippy::match_ref_pats)]
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            &Self::String => write!(formatter, "string literal"),
            &Self::Integer => write!(formatter, "integer literal"),
            &Self::True => write!(formatter, "`true`"),
            &Self::False => write!(formatter, "`false`"),
            &Self::None => write!(formatter, "`none`"),
        }
    }
}

impl GetToken for Literal {
    #[inline(always)]
    fn get_token(location: Location, buffer: &Vec<char>) -> Option<Token> {
        let content: &str = &buffer.iter().collect::<String>();

        match content {
            "true" => Some(Token {
                location,
                content: "true".to_owned(),
                token_type: TokenType::Literal(Literal::True),
            }),
            "false" => Some(Token {
                location,
                content: "false".to_owned(),
                token_type: TokenType::Literal(Literal::False),
            }),
            "none" => Some(Token {
                location,
                content: "none".to_owned(),
                token_type: TokenType::Literal(Literal::None),
            }),
            _ => None,
        }
    }
}

impl Literal {
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
    /// A result of the string as a [`Token`].
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
    /// # use lexer::tokens::token::{Location, Token, TokenType, Literal};
    /// let input: &str = "my string'"; // For lexing, the first quote has to be removed
    /// let mut iterator: std::iter::Peekable<std::iter::Enumerate<std::str::Chars>> = input.chars().enumerate().peekable();
    /// # let location: Location = Location {
    /// #    file: "<stdin>".to_owned(),
    /// #    line: 1,
    /// #    column: 1,
    /// # };
    /// assert_eq!(Literal::lex_string(&mut iterator, input, location.clone(), '\''), Ok(Token {
    ///     location,
    ///     content: "my string".to_owned(),
    ///     token_type: TokenType::Literal(Literal::String)
    /// }));
    ///
    ///
    /// ```
    ///
    /// # See also
    ///
    /// - [`Token`]
    /// - [`Literal`]
    /// - [`Literal::String`]
    #[inline(always)]
    pub fn lex_string(
        iterator: &mut std::iter::Peekable<std::iter::Enumerate<std::str::Chars>>,
        line: &str,
        location: Location,
        quote_type: char,
    ) -> Result<Token, LexerError> {
        let last_character: core::cell::Cell<char> = core::cell::Cell::new('\0');
        let second_to_last_character: core::cell::Cell<char> = core::cell::Cell::new('\0');
        let buffer: Vec<char> = iterator
            .peek_while(|&(_, next_character)| {
                let value: bool = ((second_to_last_character.get() == '\\'
                    && last_character.get() == '\\')
                    || (second_to_last_character.get() != '\\' && last_character.get() != '\\'))
                    && next_character == quote_type;
                second_to_last_character.set(last_character.get());
                last_character.set(next_character);
                !value
            })
            .iter()
            .map(|&(_, found)| found)
            .collect::<Vec<char>>();

        let help: String = format!("Add `{quote_type}` here");
        let file: String = location.file.clone();
        let snippet: annotate_snippets::Snippet = annotate_snippets::Snippet {
            title: Some(annotate_snippets::Annotation {
                id: Some("E0002"),
                label: Some("Unterminated string literal"),
                annotation_type: annotate_snippets::AnnotationType::Error,
            }),
            footer: vec![],
            slices: vec![annotate_snippets::Slice {
                source: line,
                line_start: location.line,
                origin: Some(&file),
                annotations: vec![
                    annotate_snippets::SourceAnnotation {
                        range: (location.column - 1, location.column),
                        label: "String starts here",
                        annotation_type: annotate_snippets::AnnotationType::Help,
                    },
                    if buffer.get(
                        TryInto::<usize>::try_into(buffer.len() - 2).unwrap_or(buffer.len() - 1),
                    ) == Some(&'\\')
                        && buffer.last() == Some(&quote_type)
                    {
                        annotate_snippets::SourceAnnotation {
                            range: (
                                location.column + buffer.len() - 2,
                                location.column + buffer.len() - 1,
                            ),
                            label: "Remove the `\\` here",
                            annotation_type: annotate_snippets::AnnotationType::Help,
                        }
                    } else {
                        annotate_snippets::SourceAnnotation {
                            range: (
                                location.column + buffer.len(),
                                location.column + buffer.len() + 1,
                            ),
                            label: &help,
                            annotation_type: annotate_snippets::AnnotationType::Help,
                        }
                    },
                ],
                fold: false,
            }],
        };

        if iterator.next().is_none() {
            return Err(LexerError::UnterminatedString {
                location,
                error: annotate_snippets::Renderer::styled()
                    .render(snippet)
                    .to_string(),
            });
        }

        Ok(Token {
            location,
            content: buffer.iter().collect::<String>(),
            token_type: TokenType::Literal(Literal::String),
        })
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
    Literal(Literal),
    /// A token representing a keyword, e.g. `break`.
    Keyword(Keyword),
    /// A token representing a mark, e.g. `==`.
    Mark(Mark),
    /// A token representing an identifier, e.g. `my_var`.
    Identifier,
    /// A token representing a comment, e.g. `// comment`.
    Comment,
    /// A token representing a comment across multiple lines, e.g. `/* comment */`.
    BlockComment,
}

impl core::fmt::Display for TokenType {
    #[inline]
    #[allow(clippy::pattern_type_mismatch)]
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Type(type_name) => type_name.fmt(formatter),
            Self::Literal(literal) => literal.fmt(formatter),
            Self::Keyword(keyword) => keyword.fmt(formatter),
            Self::Mark(mark) => mark.fmt(formatter),
            Self::Identifier => write!(formatter, "identifier"),
            Self::Comment => write!(formatter, "comment"),
            Self::BlockComment => write!(formatter, "block comment"),
        }
    }
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
    ) -> Result<Option<Token>, LexerError> {
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
                let file: String = location.file.clone();
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
                        origin: Some(&file),
                        annotations: vec![annotate_snippets::SourceAnnotation {
                            range: (location.column - 1, line.len() - iterator.clone().count()),
                            label: "Unterminated comment",
                            annotation_type: annotate_snippets::AnnotationType::Error,
                        }],
                        fold: false,
                    }],
                };

                return Err(LexerError::UnterminatedComment {
                    location,
                    error: annotate_snippets::Renderer::styled()
                        .render(snippet)
                        .to_string(),
                });
            }

            iterator.next();

            return Ok(Some(Token {
                location: location.clone(),
                content: buffer[..buffer.len() - 1]
                    .iter()
                    .collect::<String>()
                    .trim()
                    .to_owned(),
                token_type: TokenType::BlockComment,
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

impl Default for Location {
    #[inline]
    fn default() -> Location {
        Location {
            file: "<stdin>".to_owned(),
            line: 1,
            column: 1,
        }
    }
}

impl core::fmt::Display for Location {
    #[inline]
    #[allow(clippy::min_ident_chars)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
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

/// A token in the lexer only used to compare to [`Token`] without comparing the location.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DummyToken {
    /// The content of the token. Will only be used for matching when `token_type` is [`TokenType::Identifier`] and the content is not empty.
    pub content: String,
    /// The type of the token.
    pub token_type: TokenType,
}

impl PartialEq<Token> for DummyToken {
    #[inline]
    fn eq(&self, other: &Token) -> bool {
        if self.token_type != other.token_type {
            return false;
        }

        if self.token_type == TokenType::Identifier && !self.content.is_empty() {
            self.content == other.content
        } else {
            true
        }
    }
}

impl From<Token> for DummyToken {
    #[inline]
    fn from(value: Token) -> Self {
        DummyToken {
            content: value.content,
            token_type: value.token_type,
        }
    }
}
