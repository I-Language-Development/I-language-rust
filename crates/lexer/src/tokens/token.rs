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

use std;

use crate::tokens::constant::{Constant, Type};
use crate::tokens::keyword::Keyword;
use crate::tokens::mark::Mark;


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
    /// impl GetToken for Keyword {
    ///     fn get_token(location: Location, buffer: &Vec<char>) -> Option<Token> {
    ///         if *buffer == vec!['f', 'o', 'o'] {
    ///             Some(Token {
    ///                 location: location,
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
    Str,
    /// The integer literal. Examples: `1`, `123`, `-10`, `1_000_000`
    Integer,
    /// The float literal. Examples: `1.0`, `1.23`, `-10.0`, `1_000_000.20`
    Float,
    /// The boolean literal. The only values are `true` and `false`.
    Boolean(Constant),
    /// The none type. Also referred to as null type. The only value is `none`.
    None,
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

impl std::fmt::Display for Location {
    #[inline]
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
