// I Language lexer keyword tests.
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
    use lexer::tokens::token::GetToken;

    fn generate_test(
        location: &lexer::tokens::token::Location,
        input: &str,
        keyword: lexer::tokens::keyword::Keyword,
    ) -> bool {
        lexer::tokens::keyword::Keyword::get_token(
            location.clone(),
            &input.chars().collect::<Vec<char>>(),
        ) == Some(lexer::tokens::token::Token {
            location: location.clone(),
            content: input.to_owned(),
            token_type: lexer::tokens::token::TokenType::Keyword(keyword),
        })
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_keyword() {
        let location: lexer::tokens::token::Location = lexer::tokens::token::Location {
            file: "tests".to_owned(),
            line: 1,
            column: 1,
        };

        assert!(generate_test(
            &location,
            "break",
            lexer::tokens::keyword::Keyword::Break
        ));
        assert!(generate_test(
            &location,
            "case",
            lexer::tokens::keyword::Keyword::Case
        ));
        assert!(generate_test(
            &location,
            "catch",
            lexer::tokens::keyword::Keyword::Catch
        ));
        assert!(generate_test(
            &location,
            "class",
            lexer::tokens::keyword::Keyword::Class
        ));
        assert!(generate_test(
            &location,
            "const",
            lexer::tokens::keyword::Keyword::Const
        ));
        assert!(generate_test(
            &location,
            "continue",
            lexer::tokens::keyword::Keyword::Continue
        ));
        assert!(generate_test(
            &location,
            "default",
            lexer::tokens::keyword::Keyword::Default
        ));
        assert!(generate_test(
            &location,
            "delete",
            lexer::tokens::keyword::Keyword::Delete
        ));
        assert!(generate_test(
            &location,
            "else",
            lexer::tokens::keyword::Keyword::Else
        ));
        assert!(generate_test(
            &location,
            "finally",
            lexer::tokens::keyword::Keyword::Finally
        ));
        assert!(generate_test(
            &location,
            "for",
            lexer::tokens::keyword::Keyword::For
        ));
        assert!(generate_test(
            &location,
            "function",
            lexer::tokens::keyword::Keyword::Function
        ));
        assert!(generate_test(
            &location,
            "if",
            lexer::tokens::keyword::Keyword::If
        ));
        assert!(generate_test(
            &location,
            "import",
            lexer::tokens::keyword::Keyword::Import
        ));
        assert!(generate_test(
            &location,
            "match",
            lexer::tokens::keyword::Keyword::Match
        ));
        assert!(generate_test(
            &location,
            "pub",
            lexer::tokens::keyword::Keyword::Pub
        ));
        assert!(generate_test(
            &location,
            "return",
            lexer::tokens::keyword::Keyword::Return
        ));
        assert!(generate_test(
            &location,
            "throw",
            lexer::tokens::keyword::Keyword::Throw
        ));
        assert!(generate_test(
            &location,
            "try",
            lexer::tokens::keyword::Keyword::Try
        ));
        assert!(generate_test(
            &location,
            "use",
            lexer::tokens::keyword::Keyword::Use
        ));
        assert!(generate_test(
            &location,
            "while",
            lexer::tokens::keyword::Keyword::While
        ));
        assert_eq!(
            lexer::tokens::keyword::Keyword::get_token(
                location.clone(),
                &" ".chars().collect::<Vec<char>>()
            ),
            None
        );
    }
}
