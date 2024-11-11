// I Language lexer token tests.
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

///////////
// TESTS //
///////////

#[cfg(test)]
mod tests {
    use lexer::tokens::token::{DummyToken, GetToken, Location, Token, TokenType};

    #[test]
    fn test_literal_display() {
        use lexer::tokens::token::Literal;

        assert_eq!(&format!("{}", Literal::String), "string literal");
        assert_eq!(&format!("{}", Literal::Integer), "integer literal");
        assert_eq!(&format!("{}", Literal::True), "`true`");
        assert_eq!(&format!("{}", Literal::False), "`false`");
        assert_eq!(&format!("{}", Literal::None), "`none`");
    }

    #[test]
    fn test_literal() {
        use lexer::tokens::token::Literal;

        let location: Location = Location {
            file: "tests".to_owned(),
            line: 1,
            column: 1,
        };

        assert_eq!(
            Literal::get_token(location.clone(), &"true".chars().collect::<Vec<char>>()),
            Some(Token {
                location: location.clone(),
                content: "true".to_owned(),
                token_type: TokenType::Literal(Literal::True),
            })
        );
        assert_eq!(
            Literal::get_token(location.clone(), &"false".chars().collect::<Vec<char>>()),
            Some(Token {
                location: location.clone(),
                content: "false".to_owned(),
                token_type: TokenType::Literal(Literal::False),
            })
        );
        assert_eq!(
            Literal::get_token(location.clone(), &"none".chars().collect::<Vec<char>>()),
            Some(Token {
                location: location.clone(),
                content: "none".to_owned(),
                token_type: TokenType::Literal(Literal::None),
            })
        );
        assert_eq!(
            Literal::get_token(location.clone(), &"/".chars().collect::<Vec<char>>()),
            None
        );
    }

    #[test]
    fn test_lex_string() {
        use std;

        use lexer::tokens::token::Literal;

        let location: Location = Location {
            file: "tests".to_owned(),
            line: 1,
            column: 1,
        };

        let input: &str = "my string'"; // For lexing, the first quote has to be removed
        let mut iterator: std::iter::Peekable<std::iter::Enumerate<std::str::Chars>> =
            input.chars().enumerate().peekable();

        assert_eq!(
            Literal::lex_string(&mut iterator, input, location.clone(), '\''),
            Ok(Token {
                location,
                content: "my string".to_owned(),
                token_type: TokenType::Literal(Literal::String)
            })
        );
    }

    #[test]
    fn test_lex_mark() {
        use std;

        use lexer::tokens::mark::Mark;

        let location: Location = Location {
            file: "tests".to_owned(),
            line: 1,
            column: 1,
        };

        let input: &str = "==";
        let mut iterator: std::iter::Peekable<std::iter::Enumerate<std::str::Chars>> =
            input.chars().enumerate().peekable();

        assert_eq!(
            TokenType::lex_mark(&mut iterator, input, location.clone(), '='),
            Ok(Some(Token {
                location,
                content: "==".to_owned(),
                token_type: TokenType::Mark(Mark::Equal)
            }))
        );
    }

    #[test]
    fn test_location_debug() {
        let location: Location = Location {
            file: "tests".to_owned(),
            line: 1,
            column: 1,
        };

        assert_eq!(&format!("{location}"), "tests:1:1");
    }

    #[test]
    fn test_dummy_token() {
        let location: Location = Location {
            file: "tests".to_owned(),
            line: 1,
            column: 1,
        };

        assert!(
            DummyToken {
                content: String::new(),
                token_type: TokenType::Comment
            } == Token {
                location: location.clone(),
                content: String::new(),
                token_type: TokenType::Comment
            }
        );
        assert!(
            DummyToken {
                content: "test".to_owned(),
                token_type: TokenType::Identifier
            } == Token {
                location: location.clone(),
                content: "test".to_owned(),
                token_type: TokenType::Identifier
            }
        );
        assert!(
            DummyToken {
                content: "test".to_owned(),
                token_type: TokenType::Identifier
            } != Token {
                location,
                content: "not test".to_owned(),
                token_type: TokenType::Identifier
            }
        );
    }
}
