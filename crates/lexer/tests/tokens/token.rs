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
    use lexer::tokens::token::{GetToken, Location, Token, TokenType};

    #[test]
    fn test_literal() {
        use lexer::tokens::token::TypeDefinition;

        let location: Location = Location {
            file: "tests".to_owned(),
            line: 1,
            column: 1,
        };

        assert_eq!(
            TypeDefinition::get_token(location.clone(), &"true".chars().collect::<Vec<char>>()),
            Some(Token {
                location: location.clone(),
                content: "true".to_owned(),
                token_type: TokenType::TypeDefinition(TypeDefinition::True),
            })
        );
        assert_eq!(
            TypeDefinition::get_token(location.clone(), &"false".chars().collect::<Vec<char>>()),
            Some(Token {
                location: location.clone(),
                content: "false".to_owned(),
                token_type: TokenType::TypeDefinition(TypeDefinition::False),
            })
        );
        assert_eq!(
            TypeDefinition::get_token(location.clone(), &"none".chars().collect::<Vec<char>>()),
            Some(Token {
                location: location.clone(),
                content: "none".to_owned(),
                token_type: TokenType::TypeDefinition(TypeDefinition::None),
            })
        );
        assert_eq!(
            TypeDefinition::get_token(location.clone(), &"/".chars().collect::<Vec<char>>()),
            None
        );
    }

    #[test]
    fn test_lex_string() {
        use std;

        use lexer::tokens::token::TypeDefinition;

        let location: Location = Location {
            file: "tests".to_owned(),
            line: 1,
            column: 1,
        };

        let input: &str = "my string'"; // For lexing, the first quote has to be removed
        let mut iterator: std::iter::Peekable<std::iter::Enumerate<std::str::Chars>> =
            input.chars().enumerate().peekable();

        assert_eq!(
            TypeDefinition::lex_string(&mut iterator, input, location.clone(), '\''),
            Token {
                location,
                content: "my string".to_owned(),
                token_type: TokenType::TypeDefinition(TypeDefinition::String)
            }
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
            Some(Token {
                location,
                content: "==".to_owned(),
                token_type: TokenType::Mark(Mark::Equal)
            })
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
}
