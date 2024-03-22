//! The lexers marks, which are tokens like `+`, `-` or `=`.
// I Language lexer marks.
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


//////////
// MARK //
//////////

/// Mark tokens representing a mark (`+`, `=`, etc.) in the lexer.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Mark {
    /// The `+` mark. Used for adding the left and right values together.
    Add,
    /// The `+=` mark. Used for adding the right value to the variable and assigning that value to the variable.
    AddAssign,
    /// The `&&` (**logical** and) mark. Used for creating a true/false value based on whether both the left and right tokens are true.
    And,
    /// The `=` mark. Used for assigning a value to a variable.
    Assign,
    /// The `@` mark. Currently, it has no use, but it's reserved for later usage and will probably be used in future.
    At,
    /// The `!` (**logical** not) mark. Used for flipping a true value to false and vice versa.
    Bang,
    /// The `&` (**bitwise** and) mark. Used for performing the bitwise-and operation with the left and right tokens.
    BitAnd,
    /// The `&=` (**bitwise** and assign) mark. Used for performing the bitwise-and operation with the value stored in the variable and the value of the right token and assigning that value to the variable.
    BitAndAssign,
    /// The `~` (**bitwise** not) mark. Used for performing the bitwise-not operation with the left and right tokens.
    BitNot,
    /// The `~=` (**bitwise** not assign) mark. Used for performing the bitwise-not operation with the value stored in the variable and the value of the right token and assigning that value to the variable.
    BitNotAssign,
    /// The `|` (**bitwise** or) mark. Used for performing the bitwise-or operation with the left and right tokens.
    BitOr,
    /// The `|=` (**bitwise** or assign) mark. Used for performing the bitwise-or operation with the value stored in the variable and the value of the right token and assigning that value to the variable.
    BitOrAssign,
    /// The `^` (**bitwise** exclusive or) mark. Used for performing the bitwise-xor operation with the left and right tokens.
    BitXor,
    /// The `^=` (**bitwise** exclusive or assign) mark. Used for performing the bitwise-xor operation with the value stored in the variable and the value of the right token and assigning that value to the variable.
    BitXorAssign,
    /// The `{` mark. Used for opening a code block and defining a set and dictionary (hashmap).
    BraceOpen,
    /// The `}` mark. Used for closing a code block and defining a set and dictionary (hashmap).
    BraceClose,
    /// The `[` mark. Used for indexing a value and opening an array.
    BracketOpen,
    /// The `]` mark. Used for indexing a value and closing an array.
    BracketClose,
    /// The `:` mark. Used for defining type parameters and as a separator between a dictionary's key and value.
    Colon,
    /// The `,` mark. Used for adding more items to a data structure.
    Comma,
    /// The `--` mark. Used for decreasing the left token by one.
    Decrease,
    /// The `/` mark. Used for dividing the left token by the right token.
    Divide,
    /// The `/=` mark. Used for dividing the variable by the right token and assigning that value to the variable.
    DivideAssign,
    /// The `.` mark. Used for accessing sub items of a scope and starting floats.
    Dot,
    /// The `==` mark. Used for checking if the left and right tokens are equal.
    Equal,
    /// The `**` mark. Used for exponentiation with the left token with the right token.
    Exponentiation,
    /// The `>` mark. Used for checking whether the left value is greater than the right value.
    Greater,
    /// The `>=` mark. Used for checking whether the left value is greater than or the same as the right value.
    GreaterEqual,
    /// The `++` mark. Used for increasing the left token by one.
    Increase,
    /// The `<` mark. Used for checking whether the left value is smaller than the right value.
    Less,
    /// The `<=` mark. Used for checking whether the left value is smaller than or the same as the right value.
    LessEqual,
    /// The `%` mark. Used for performing a modulo operation on the left and right values.
    Modulo,
    /// The `%=` mark. Used for performing a modulo operation on the variable and the right value and assigning that value to the variable.
    ModuloAssign,
    /// The `*` mark. Used for multiplying the left and right values.
    Multiply,
    /// The `*=` mark. Used for multiplying the variable and the right value and assigning that value to the variable.
    MultiplyAssign,
    /// The `!=` mark. Used for checking if the left and right values are *not* equal.
    NotEqual,
    /// The `||` (**logical** or) mark. Used for checking if either the left or the right values are true. If both are true, it will still return true.
    Or,
    /// The `(` mark. Used for function parameters and operator precedence for mathematical operations.
    ParenthesisOpen,
    /// The `)` mark. Used for function parameters and operator precedence for mathematical operations.
    ParenthesisClose,
    /// The `?` mark. Currently, it has no use, but it's reserved for later usage and will probably be used in future.
    QuestionMark,
    /// The `..` mark. Used to create a range for indexing or looping. Currently, only ranges with spaces around them are supported, `1..2` will not work and returns a float.
    Range,
    /// The `;` mark. Used to end a line of code.
    Semicolon,
    /// The `<<` (**bitwise** left shift) mark. Used to shift bytes n digits to the left, where n is the right value.
    ShiftLeft,
    /// The `<<=` (**bitwise** left shift assign) mark. Used to shift bytes n digits to the left, where n is the right value, and assigning that value to the variable.
    ShiftLeftAssign,
    /// The `>>` (**bitwise** right shift) mark. Used to shift bytes n digits to the right, where n is the right value.
    ShiftRight,
    /// The `>>=` (**bitwise** right shift assign) mark. Used to shift bytes n digits to the right, where n is the right value, and assigning that value to the variable.
    ShiftRightAssign,
    /// The `-` mark. Used for subtracting the right value from the left value.
    Subtract,
    /// The `-=` mark. Used for subtracting the right value from the variable and assigning that value to the variable.
    SubtractAssign,
}

impl core::fmt::Display for Mark {
    #[inline]
    #[allow(clippy::match_ref_pats)]
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            &Self::Add => write!(formatter, "+"),
            &Self::AddAssign => write!(formatter, "+="),
            &Self::And => write!(formatter, "&&"),
            &Self::Assign => write!(formatter, "="),
            &Self::At => write!(formatter, "@"),
            &Self::Bang => write!(formatter, "!"),
            &Self::BitAnd => write!(formatter, "&"),
            &Self::BitAndAssign => write!(formatter, "&="),
            &Self::BitNot => write!(formatter, "~"),
            &Self::BitNotAssign => write!(formatter, "~="),
            &Self::BitOr => write!(formatter, "|"),
            &Self::BitOrAssign => write!(formatter, "|="),
            &Self::BitXor => write!(formatter, "^"),
            &Self::BitXorAssign => write!(formatter, "^="),
            &Self::BraceOpen => write!(formatter, "{{"),
            &Self::BraceClose => write!(formatter, "}}"),
            &Self::BracketOpen => write!(formatter, "["),
            &Self::BracketClose => write!(formatter, "]"),
            &Self::Colon => write!(formatter, ":"),
            &Self::Comma => write!(formatter, ","),
            &Self::Decrease => write!(formatter, "--"),
            &Self::Divide => write!(formatter, "/"),
            &Self::DivideAssign => write!(formatter, "/="),
            &Self::Dot => write!(formatter, "."),
            &Self::Equal => write!(formatter, "=="),
            &Self::Exponentiation => write!(formatter, "**"),
            &Self::Greater => write!(formatter, ">"),
            &Self::GreaterEqual => write!(formatter, ">="),
            &Self::Increase => write!(formatter, "++"),
            &Self::Less => write!(formatter, "<"),
            &Self::LessEqual => write!(formatter, "<="),
            &Self::Modulo => write!(formatter, "%"),
            &Self::ModuloAssign => write!(formatter, "%="),
            &Self::Multiply => write!(formatter, "*"),
            &Self::MultiplyAssign => write!(formatter, "*="),
            &Self::NotEqual => write!(formatter, "!="),
            &Self::Or => write!(formatter, "||"),
            &Self::ParenthesisOpen => write!(formatter, "("),
            &Self::ParenthesisClose => write!(formatter, ")"),
            &Self::QuestionMark => write!(formatter, "?"),
            &Self::Range => write!(formatter, ".."),
            &Self::Semicolon => write!(formatter, ";"),
            &Self::ShiftLeft => write!(formatter, "<<"),
            &Self::ShiftLeftAssign => write!(formatter, "<<="),
            &Self::ShiftRight => write!(formatter, ">>"),
            &Self::ShiftRightAssign => write!(formatter, ">>="),
            &Self::Subtract => write!(formatter, "-"),
            &Self::SubtractAssign => write!(formatter, "-="),
        }
    }
}

impl GetToken for Mark {
    #[inline]
    #[allow(clippy::too_many_lines)]
    fn get_token(location: Location, buffer: &Vec<char>) -> Option<Token> {
        let content: &str = &buffer.iter().collect::<String>();

        match content {
            "+" => Some(Token {
                location,
                content: "+".to_owned(),
                token_type: TokenType::Mark(Mark::Add),
            }),
            "+=" => Some(Token {
                location,
                content: "+=".to_owned(),
                token_type: TokenType::Mark(Mark::AddAssign),
            }),
            "&&" => Some(Token {
                location,
                content: "&&".to_owned(),
                token_type: TokenType::Mark(Mark::And),
            }),
            "=" => Some(Token {
                location,
                content: "=".to_owned(),
                token_type: TokenType::Mark(Mark::Assign),
            }),
            "@" => Some(Token {
                location,
                content: "@".to_owned(),
                token_type: TokenType::Mark(Mark::At),
            }),
            "!" => Some(Token {
                location,
                content: "!".to_owned(),
                token_type: TokenType::Mark(Mark::Bang),
            }),
            "&" => Some(Token {
                location,
                content: "&".to_owned(),
                token_type: TokenType::Mark(Mark::BitAnd),
            }),
            "&=" => Some(Token {
                location,
                content: "&=".to_owned(),
                token_type: TokenType::Mark(Mark::BitAndAssign),
            }),
            "~" => Some(Token {
                location,
                content: "~".to_owned(),
                token_type: TokenType::Mark(Mark::BitNot),
            }),
            "~=" => Some(Token {
                location,
                content: "~=".to_owned(),
                token_type: TokenType::Mark(Mark::BitNotAssign),
            }),
            "|" => Some(Token {
                location,
                content: "|".to_owned(),
                token_type: TokenType::Mark(Mark::BitOr),
            }),
            "|=" => Some(Token {
                location,
                content: "|=".to_owned(),
                token_type: TokenType::Mark(Mark::BitOrAssign),
            }),
            "^" => Some(Token {
                location,
                content: "^".to_owned(),
                token_type: TokenType::Mark(Mark::BitXor),
            }),
            "^=" => Some(Token {
                location,
                content: "^=".to_owned(),
                token_type: TokenType::Mark(Mark::BitXorAssign),
            }),
            "{" => Some(Token {
                location,
                content: "{".to_owned(),
                token_type: TokenType::Mark(Mark::BraceOpen),
            }),
            "}" => Some(Token {
                location,
                content: "}".to_owned(),
                token_type: TokenType::Mark(Mark::BraceClose),
            }),
            "[" => Some(Token {
                location,
                content: "[".to_owned(),
                token_type: TokenType::Mark(Mark::BracketOpen),
            }),
            "]" => Some(Token {
                location,
                content: "]".to_owned(),
                token_type: TokenType::Mark(Mark::BracketClose),
            }),
            ":" => Some(Token {
                location,
                content: ":".to_owned(),
                token_type: TokenType::Mark(Mark::Colon),
            }),
            "," => Some(Token {
                location,
                content: ",".to_owned(),
                token_type: TokenType::Mark(Mark::Comma),
            }),
            "--" => Some(Token {
                location,
                content: "--".to_owned(),
                token_type: TokenType::Mark(Mark::Decrease),
            }),
            "/" => Some(Token {
                location,
                content: "/".to_owned(),
                token_type: TokenType::Mark(Mark::Divide),
            }),
            "/=" => Some(Token {
                location,
                content: "/=".to_owned(),
                token_type: TokenType::Mark(Mark::DivideAssign),
            }),
            "." => Some(Token {
                location,
                content: ".".to_owned(),
                token_type: TokenType::Mark(Mark::Dot),
            }),
            "==" => Some(Token {
                location,
                content: "==".to_owned(),
                token_type: TokenType::Mark(Mark::Equal),
            }),
            "**" => Some(Token {
                location,
                content: "**".to_owned(),
                token_type: TokenType::Mark(Mark::Exponentiation),
            }),
            ">" => Some(Token {
                location,
                content: ">".to_owned(),
                token_type: TokenType::Mark(Mark::Greater),
            }),
            ">=" => Some(Token {
                location,
                content: ">=".to_owned(),
                token_type: TokenType::Mark(Mark::GreaterEqual),
            }),
            "++" => Some(Token {
                location,
                content: "++".to_owned(),
                token_type: TokenType::Mark(Mark::Increase),
            }),
            "<" => Some(Token {
                location,
                content: "<".to_owned(),
                token_type: TokenType::Mark(Mark::Less),
            }),
            "<=" => Some(Token {
                location,
                content: "<=".to_owned(),
                token_type: TokenType::Mark(Mark::LessEqual),
            }),
            "%" => Some(Token {
                location,
                content: "%".to_owned(),
                token_type: TokenType::Mark(Mark::Modulo),
            }),
            "%=" => Some(Token {
                location,
                content: "%=".to_owned(),
                token_type: TokenType::Mark(Mark::ModuloAssign),
            }),
            "*" => Some(Token {
                location,
                content: "*".to_owned(),
                token_type: TokenType::Mark(Mark::Multiply),
            }),
            "*=" => Some(Token {
                location,
                content: "*=".to_owned(),
                token_type: TokenType::Mark(Mark::MultiplyAssign),
            }),
            "!=" => Some(Token {
                location,
                content: "!=".to_owned(),
                token_type: TokenType::Mark(Mark::NotEqual),
            }),
            "||" => Some(Token {
                location,
                content: "||".to_owned(),
                token_type: TokenType::Mark(Mark::Or),
            }),
            "(" => Some(Token {
                location,
                content: "(".to_owned(),
                token_type: TokenType::Mark(Mark::ParenthesisOpen),
            }),
            ")" => Some(Token {
                location,
                content: ")".to_owned(),
                token_type: TokenType::Mark(Mark::ParenthesisClose),
            }),
            "?" => Some(Token {
                location,
                content: "?".to_owned(),
                token_type: TokenType::Mark(Mark::QuestionMark),
            }),
            ".." => Some(Token {
                location,
                content: "..".to_owned(),
                token_type: TokenType::Mark(Mark::Range),
            }),
            ";" => Some(Token {
                location,
                content: ";".to_owned(),
                token_type: TokenType::Mark(Mark::Semicolon),
            }),
            "<<" => Some(Token {
                location,
                content: "<<".to_owned(),
                token_type: TokenType::Mark(Mark::ShiftLeft),
            }),
            "<<=" => Some(Token {
                location,
                content: "<<=".to_owned(),
                token_type: TokenType::Mark(Mark::ShiftLeftAssign),
            }),
            ">>" => Some(Token {
                location,
                content: ">>".to_owned(),
                token_type: TokenType::Mark(Mark::ShiftRight),
            }),
            ">>=" => Some(Token {
                location,
                content: ">>=".to_owned(),
                token_type: TokenType::Mark(Mark::ShiftRightAssign),
            }),
            "-" => Some(Token {
                location,
                content: "-".to_owned(),
                token_type: TokenType::Mark(Mark::Subtract),
            }),
            "-=" => Some(Token {
                location,
                content: "-=".to_owned(),
                token_type: TokenType::Mark(Mark::SubtractAssign),
            }),
            _ => None,
        }
    }
}
