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
    use lexer::tokens::keyword::Keyword;
    use lexer::tokens::token::{GetToken, Location, Token, TokenType};

    fn generate_test(location: &Location, input: &str, keyword: Keyword) -> bool {
        Keyword::get_token(location.clone(), &input.chars().collect::<Vec<char>>())
            == Some(Token {
                location: location.clone(),
                content: input.to_owned(),
                token_type: TokenType::Keyword(keyword),
            })
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_keyword() {
        let location: Location = Location {
            file: "tests".to_owned(),
            line: 1,
            column: 1,
        };

        assert!(generate_test(&location, "as", Keyword::As));
        assert!(generate_test(&location, "break", Keyword::Break));
        assert!(generate_test(&location, "case", Keyword::Case));
        assert!(generate_test(&location, "catch", Keyword::Catch));
        assert!(generate_test(&location, "class", Keyword::Class));
        assert!(generate_test(&location, "const", Keyword::Const));
        assert!(generate_test(&location, "continue", Keyword::Continue));
        assert!(generate_test(&location, "default", Keyword::Default));
        assert!(generate_test(&location, "else", Keyword::Else));
        assert!(generate_test(&location, "finally", Keyword::Finally));
        assert!(generate_test(&location, "for", Keyword::For));
        assert!(generate_test(&location, "function", Keyword::Function));
        assert!(generate_test(&location, "if", Keyword::If));
        assert!(generate_test(&location, "import", Keyword::Import));
        assert!(generate_test(&location, "match", Keyword::Match));
        assert!(generate_test(&location, "pub", Keyword::Pub));
        assert!(generate_test(&location, "return", Keyword::Return));
        assert!(generate_test(&location, "throw", Keyword::Throw));
        assert!(generate_test(&location, "try", Keyword::Try));
        assert!(generate_test(&location, "use", Keyword::Use));
        assert!(generate_test(&location, "var", Keyword::Var));
        assert!(generate_test(&location, "while", Keyword::While));
        assert_eq!(
            Keyword::get_token(location.clone(), &" ".chars().collect::<Vec<char>>()),
            None
        );
    }
}
