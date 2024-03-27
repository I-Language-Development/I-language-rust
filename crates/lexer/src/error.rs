//! Errors that may occur while lexing.
// I Language lexer errors.
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

use crate::tokens::token::Location;

use thiserror::Error;


////////////
// ERRORS //
////////////

/// The different kinds of errors the lexer can raise.
/// All of these errors implement [`std::error::Error`].
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
pub enum LexerError {
    /// An error which will be returned if a mark was invalid for some reason.
    /// This can occur when the starting character of a mark is valid, but the character after it is not.
    #[error("invalid mark at {location}")]
    InvalidMark { location: Location },

    /// An error which will be returned if an unexpected character is encountered.
    /// this is most likely to occur when using unicode characters as they are not supported.
    #[error("unexpected character `{character}` at {location}")]
    UnexpectedCharacter { character: char, location: Location },

    /// An error which will be returned if a comment is not terminated by a closing `*/`.
    #[error("unterminated comment at {location}")]
    UnterminatedComment { location: Location },

    /// An error which will be returned if a string is not terminated by a closing quote or the quote is escaped.
    #[error("unterminated string at {location}")]
    UnterminatedString { location: Location },
}
