// I Language lexer tests.
// Version: 1.0.0
//
// Copyright (c) 2023-present I Language Development.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the 'Software'),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
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
            I_Language_lexer::lex::lex("1 + 1", "<stdin>"),
            [
                I_Language_lexer::tokens::token::Token {
                    location: I_Language_lexer::tokens::token::Location {
                        file: "<stdin>".to_owned(),
                        line: 1,
                        column: 1,
                    },
                    content: "1".to_owned(),
                    token_type: I_Language_lexer::tokens::token::TokenType::TypeDefinition(
                        I_Language_lexer::tokens::token::TypeDefinition::Integer,
                    ),
                },
                I_Language_lexer::tokens::token::Token {
                    location: I_Language_lexer::tokens::token::Location {
                        file: "<stdin>".to_owned(),
                        line: 1,
                        column: 3,
                    },
                    content: "+".to_owned(),
                    token_type: I_Language_lexer::tokens::token::TokenType::Mark(
                        I_Language_lexer::tokens::mark::Mark::Add,
                    ),
                },
                I_Language_lexer::tokens::token::Token {
                    location: I_Language_lexer::tokens::token::Location {
                        file: "<stdin>".to_owned(),
                        line: 1,
                        column: 5,
                    },
                    content: "1".to_owned(),
                    token_type: I_Language_lexer::tokens::token::TokenType::TypeDefinition(
                        I_Language_lexer::tokens::token::TypeDefinition::Integer,
                    ),
                },
            ]
        );
    }
}
