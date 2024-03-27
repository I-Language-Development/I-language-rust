//! The lexers keywords, which are tokens like `break`, `if` or `class`.
// I Language lexer keywords.
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

use crate::tokens::token::{GetToken, Location, Token, TokenType};


//////////////
// KEYWORDS //
//////////////

/// Keyword tokens representing a keyword (`if`, `class`, etc.) in the lexer.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Keyword {
    /// The `as` keyword. Used with the [`import`](`Keyword::Import`) keyword to import items into a specific namespace.
    As,
    /// The `break` keyword. Used for exiting out of a loop.
    Break,
    /// The `case` keyword. Used in combination with the [`match`](`Keyword::Match`) keyword to match a specific case.
    Case,
    /// The `catch` keyword. Used in combination with the [`try`](`Keyword::Try`) keyword to catch exceptions and implement error handling.
    Catch,
    /// The `class` keyword. Used to define classes.
    Class,
    /// The `const` keyword. Used to define a variable as constant.
    Const,
    /// The `continue` keyword. Used to continue a loop before all of it's code is executed.
    Continue,
    /// The `else` keyword. Used to define the "otherwise" block of an [`if`](`Keyword::If`) statement.
    Else,
    /// The `finally` keyword. Used in combination with the [`try`](`Keyword::Try`) keyword to execute code even after an exception has been raised.
    Finally,
    /// The `for` keyword. Used to create a loop over an iterator.
    For,
    /// The `if` keyword. Used to check whether a condition is true or false and execute code based on that condition.
    If,
    /// The `import` keyword. Used to import code from other modules.
    Import,
    /// The `match` keyword. Used in combination with the [`case`](`Keyword::Case`) keyword.
    Match,
    /// The `pub` keyword. Used to export an item out of the current scope.
    Pub,
    /// The `return` keyword. Used to return something from a function.
    Return,
    /// The `throw` keyword. Used to throw (raise) an exception.
    Throw,
    /// The `try` keyword. Used in combination with the [`catch`](`Keyword::Catch`) keyword to catch exceptions.
    Try,
    /// The `use` keyword. Used to enable language features.
    Use,
    /// The `var` keyword. Used to define a variable without binding it to a type.
    Var,
    /// The `while` keyword. Used to crate a loop which will iterates as long as a condition is true.
    While,
}

impl core::fmt::Display for Keyword {
    #[inline]
    #[allow(clippy::match_ref_pats)]
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            &Self::As => write!(formatter, "as"),
            &Self::Break => write!(formatter, "break"),
            &Self::Case => write!(formatter, "case"),
            &Self::Catch => write!(formatter, "catch"),
            &Self::Class => write!(formatter, "class"),
            &Self::Const => write!(formatter, "const"),
            &Self::Continue => write!(formatter, "continue"),
            &Self::Else => write!(formatter, "else"),
            &Self::Finally => write!(formatter, "finally"),
            &Self::For => write!(formatter, "for"),
            &Self::If => write!(formatter, "if"),
            &Self::Import => write!(formatter, "import"),
            &Self::Match => write!(formatter, "match"),
            &Self::Pub => write!(formatter, "pub"),
            &Self::Return => write!(formatter, "return"),
            &Self::Throw => write!(formatter, "throw"),
            &Self::Try => write!(formatter, "try"),
            &Self::Use => write!(formatter, "use"),
            &Self::Var => write!(formatter, "var"),
            &Self::While => write!(formatter, "while"),
        }
    }
}

impl GetToken for Keyword {
    #[inline]
    #[allow(clippy::too_many_lines)]
    fn get_token(location: Location, buffer: &Vec<char>) -> Option<Token> {
        let content: &str = &buffer.iter().collect::<String>();

        match content {
            "as" => Some(Token {
                location,
                content: "as".to_owned(),
                token_type: TokenType::Keyword(Keyword::As),
            }),
            "break" => Some(Token {
                location,
                content: "break".to_owned(),
                token_type: TokenType::Keyword(Keyword::Break),
            }),
            "case" => Some(Token {
                location,
                content: "case".to_owned(),
                token_type: TokenType::Keyword(Keyword::Case),
            }),
            "catch" => Some(Token {
                location,
                content: "catch".to_owned(),
                token_type: TokenType::Keyword(Keyword::Catch),
            }),
            "class" => Some(Token {
                location,
                content: "class".to_owned(),
                token_type: TokenType::Keyword(Keyword::Class),
            }),
            "const" => Some(Token {
                location,
                content: "const".to_owned(),
                token_type: TokenType::Keyword(Keyword::Const),
            }),
            "continue" => Some(Token {
                location,
                content: "continue".to_owned(),
                token_type: TokenType::Keyword(Keyword::Continue),
            }),
            "else" => Some(Token {
                location,
                content: "else".to_owned(),
                token_type: TokenType::Keyword(Keyword::Else),
            }),
            "finally" => Some(Token {
                location,
                content: "finally".to_owned(),
                token_type: TokenType::Keyword(Keyword::Finally),
            }),
            "for" => Some(Token {
                location,
                content: "for".to_owned(),
                token_type: TokenType::Keyword(Keyword::For),
            }),
            "if" => Some(Token {
                location,
                content: "if".to_owned(),
                token_type: TokenType::Keyword(Keyword::If),
            }),
            "import" => Some(Token {
                location,
                content: "import".to_owned(),
                token_type: TokenType::Keyword(Keyword::Import),
            }),
            "match" => Some(Token {
                location,
                content: "match".to_owned(),
                token_type: TokenType::Keyword(Keyword::Match),
            }),
            "pub" => Some(Token {
                location,
                content: "pub".to_owned(),
                token_type: TokenType::Keyword(Keyword::Pub),
            }),
            "return" => Some(Token {
                location,
                content: "return".to_owned(),
                token_type: TokenType::Keyword(Keyword::Return),
            }),
            "throw" => Some(Token {
                location,
                content: "throw".to_owned(),
                token_type: TokenType::Keyword(Keyword::Throw),
            }),
            "try" => Some(Token {
                location,
                content: "try".to_owned(),
                token_type: TokenType::Keyword(Keyword::Try),
            }),
            "use" => Some(Token {
                location,
                content: "use".to_owned(),
                token_type: TokenType::Keyword(Keyword::Use),
            }),
            "var" => Some(Token {
                location,
                content: "var".to_owned(),
                token_type: TokenType::Keyword(Keyword::Var),
            }),
            "while" => Some(Token {
                location,
                content: "while".to_owned(),
                token_type: TokenType::Keyword(Keyword::While),
            }),
            _ => None,
        }
    }
}
