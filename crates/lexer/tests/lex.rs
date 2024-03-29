// I Language lexer tests.
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
    #[test]
    fn test_lex() {
        assert_eq!(
            lexer::lex::lex("1 + 1", "<stdin>"),
            Ok(vec![
                lexer::tokens::token::Token {
                    location: lexer::tokens::token::Location {
                        file: "<stdin>".to_owned(),
                        line: 1,
                        column: 1,
                    },
                    content: "1".to_owned(),
                    token_type: lexer::tokens::token::TokenType::TypeDefinition(
                        lexer::tokens::token::TypeDefinition::Integer,
                    ),
                },
                lexer::tokens::token::Token {
                    location: lexer::tokens::token::Location {
                        file: "<stdin>".to_owned(),
                        line: 1,
                        column: 3,
                    },
                    content: "+".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Mark(
                        lexer::tokens::mark::Mark::Add,
                    ),
                },
                lexer::tokens::token::Token {
                    location: lexer::tokens::token::Location {
                        file: "<stdin>".to_owned(),
                        line: 1,
                        column: 5,
                    },
                    content: "1".to_owned(),
                    token_type: lexer::tokens::token::TokenType::TypeDefinition(
                        lexer::tokens::token::TypeDefinition::Integer,
                    ),
                },
            ])
        );

        assert_eq!(
            lexer::lex::lex("my/* cool */code // works", "<stdin>"),
            Ok(vec![
                lexer::tokens::token::Token {
                    location: lexer::tokens::token::Location {
                        file: "<stdin>".to_owned(),
                        line: 1,
                        column: 1,
                    },
                    content: "my".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Identifier,
                },
                lexer::tokens::token::Token {
                    location: lexer::tokens::token::Location {
                        file: "<stdin>".to_owned(),
                        line: 1,
                        column: 3,
                    },
                    content: "cool".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Comment,
                },
                lexer::tokens::token::Token {
                    location: lexer::tokens::token::Location {
                        file: "<stdin>".to_owned(),
                        line: 1,
                        column: 13,
                    },
                    content: "code".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Identifier,
                },
                lexer::tokens::token::Token {
                    location: lexer::tokens::token::Location {
                        file: "<stdin>".to_owned(),
                        line: 1,
                        column: 18,
                    },
                    content: "works".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Comment,
                },
            ])
        );
    }
}
