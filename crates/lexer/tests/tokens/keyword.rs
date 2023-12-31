// I Language lexer keyword tests.
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
    use lexer::tokens::token::GetToken;

    #[test]
    fn test_keyword() {
        let location: lexer::tokens::token::Location = lexer::tokens::token::Location {
            file: "tests".to_owned(),
            line: 1,
            column: 1,
        };

        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"break".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "break".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::Break
                    ),
                },
                5
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"case".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "case".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::Case
                    ),
                },
                4
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"catch".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "catch".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::Catch
                    ),
                },
                5
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"class".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "class".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::Class
                    ),
                },
                5
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"continue".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "continue".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::Continue
                    ),
                },
                8
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"default".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "default".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::Default
                    ),
                },
                7
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"delete".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "delete".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::Delete
                    ),
                },
                6
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"else".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "else".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::Else
                    ),
                },
                4
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"finally".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "finally".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::Finally
                    ),
                },
                7
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"for".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "for".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::For
                    ),
                },
                3
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"function".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "function".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::Function
                    ),
                },
                8
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"if".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "if".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::If
                    ),
                },
                2
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"import".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "import".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::Import
                    ),
                },
                6
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"match".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "match".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::Match
                    ),
                },
                5
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"pub".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "pub".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::Pub
                    ),
                },
                3
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"return".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "return".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::Return
                    ),
                },
                6
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"throw".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "throw".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::Throw
                    ),
                },
                5
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"try".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "try".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::Try
                    ),
                },
                3
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"use".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "use".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::Use
                    ),
                },
                3
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"while".chars().collect::<Vec<char>>()
            ),
            Some((
                lexer::tokens::token::Token {
                    location: location.clone(),
                    content: "while".to_owned(),
                    token_type: lexer::tokens::token::TokenType::Keyword(
                        lexer::tokens::keyword::Keyword::While
                    ),
                },
                5
            ))
        );
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &"/".chars().collect::<Vec<char>>()
            ),
            None
        );
    }
}
