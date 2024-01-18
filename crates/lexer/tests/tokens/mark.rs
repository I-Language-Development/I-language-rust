// I Language lexer mark tests.
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
        mark: lexer::tokens::mark::Mark,
    ) -> bool {
        lexer::tokens::mark::Mark::get_token(
            location.clone(),
            &input.chars().collect::<Vec<char>>(),
        ) == Some(lexer::tokens::token::Token {
            location: location.clone(),
            content: input.to_owned(),
            token_type: lexer::tokens::token::TokenType::Mark(mark),
        })
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_mark() {
        let location: lexer::tokens::token::Location = lexer::tokens::token::Location {
            file: "tests".to_owned(),
            line: 1,
            column: 1,
        };

        assert!(generate_test(
            &location,
            "+",
            lexer::tokens::mark::Mark::Add
        ));
        assert!(generate_test(
            &location,
            "+=",
            lexer::tokens::mark::Mark::AddAssign
        ));
        assert!(generate_test(
            &location,
            "&&",
            lexer::tokens::mark::Mark::And
        ));
        assert!(generate_test(
            &location,
            "->",
            lexer::tokens::mark::Mark::Arrow
        ));
        assert!(generate_test(
            &location,
            "=",
            lexer::tokens::mark::Mark::Assign
        ));
        assert!(generate_test(&location, "@", lexer::tokens::mark::Mark::At));
        assert!(generate_test(
            &location,
            "!",
            lexer::tokens::mark::Mark::Bang
        ));
        assert!(generate_test(
            &location,
            "&",
            lexer::tokens::mark::Mark::BitAnd
        ));
        assert!(generate_test(
            &location,
            "&=",
            lexer::tokens::mark::Mark::BitAndAssign
        ));
        assert!(generate_test(
            &location,
            "~",
            lexer::tokens::mark::Mark::BitNot
        ));
        assert!(generate_test(
            &location,
            "~=",
            lexer::tokens::mark::Mark::BitNotAssign
        ));
        assert!(generate_test(
            &location,
            "|",
            lexer::tokens::mark::Mark::BitOr
        ));
        assert!(generate_test(
            &location,
            "|=",
            lexer::tokens::mark::Mark::BitOrAssign
        ));
        assert!(generate_test(
            &location,
            "^",
            lexer::tokens::mark::Mark::BitXor
        ));
        assert!(generate_test(
            &location,
            "^=",
            lexer::tokens::mark::Mark::BitXorAssign
        ));
        assert!(generate_test(
            &location,
            "{",
            lexer::tokens::mark::Mark::BraceOpen
        ));
        assert!(generate_test(
            &location,
            "}",
            lexer::tokens::mark::Mark::BraceClose
        ));
        assert!(generate_test(
            &location,
            "[",
            lexer::tokens::mark::Mark::BracketOpen
        ));
        assert!(generate_test(
            &location,
            "]",
            lexer::tokens::mark::Mark::BracketClose
        ));
        assert!(generate_test(
            &location,
            ":",
            lexer::tokens::mark::Mark::Colon
        ));
        assert!(generate_test(
            &location,
            ",",
            lexer::tokens::mark::Mark::Comma
        ));
        assert!(generate_test(
            &location,
            "--",
            lexer::tokens::mark::Mark::Decrease
        ));
        assert!(generate_test(
            &location,
            "/",
            lexer::tokens::mark::Mark::Divide
        ));
        assert!(generate_test(
            &location,
            "/=",
            lexer::tokens::mark::Mark::DivideAssign
        ));
        assert!(generate_test(
            &location,
            ".",
            lexer::tokens::mark::Mark::Dot
        ));
        assert!(generate_test(
            &location,
            "==",
            lexer::tokens::mark::Mark::Equal
        ));
        assert!(generate_test(
            &location,
            "**",
            lexer::tokens::mark::Mark::Exponentiation
        ));
        assert!(generate_test(
            &location,
            ">",
            lexer::tokens::mark::Mark::Greater
        ));
        assert!(generate_test(
            &location,
            ">=",
            lexer::tokens::mark::Mark::GreaterEqual
        ));
        assert!(generate_test(
            &location,
            "++",
            lexer::tokens::mark::Mark::Increase
        ));
        assert!(generate_test(
            &location,
            "<",
            lexer::tokens::mark::Mark::Less
        ));
        assert!(generate_test(
            &location,
            "<=",
            lexer::tokens::mark::Mark::LessEqual
        ));
        assert!(generate_test(
            &location,
            "%",
            lexer::tokens::mark::Mark::Modulo
        ));
        assert!(generate_test(
            &location,
            "%=",
            lexer::tokens::mark::Mark::ModuloAssign
        ));
        assert!(generate_test(
            &location,
            "*",
            lexer::tokens::mark::Mark::Multiply
        ));
        assert!(generate_test(
            &location,
            "*=",
            lexer::tokens::mark::Mark::MultiplyAssign
        ));
        assert!(generate_test(
            &location,
            "!=",
            lexer::tokens::mark::Mark::NotEqual
        ));
        assert!(generate_test(
            &location,
            "||",
            lexer::tokens::mark::Mark::Or
        ));
        assert!(generate_test(
            &location,
            "(",
            lexer::tokens::mark::Mark::ParenthesisOpen
        ));
        assert!(generate_test(
            &location,
            ")",
            lexer::tokens::mark::Mark::ParenthesisClose
        ));
        assert!(generate_test(
            &location,
            "?",
            lexer::tokens::mark::Mark::QuestionMark
        ));
        assert!(generate_test(
            &location,
            "..",
            lexer::tokens::mark::Mark::Range
        ));
        assert!(generate_test(
            &location,
            ";",
            lexer::tokens::mark::Mark::Semicolon
        ));
        assert!(generate_test(
            &location,
            "<<",
            lexer::tokens::mark::Mark::ShiftLeft
        ));
        assert!(generate_test(
            &location,
            "<<=",
            lexer::tokens::mark::Mark::ShiftLeftAssign
        ));
        assert!(generate_test(
            &location,
            ">>",
            lexer::tokens::mark::Mark::ShiftRight
        ));
        assert!(generate_test(
            &location,
            ">>=",
            lexer::tokens::mark::Mark::ShiftRightAssign
        ));
        assert!(generate_test(
            &location,
            "-",
            lexer::tokens::mark::Mark::Subtract
        ));
        assert!(generate_test(
            &location,
            "-=",
            lexer::tokens::mark::Mark::SubtractAssign
        ));
        assert_eq!(
            lexer::tokens::mark::Mark::get_token(
                location.clone(),
                &" ".chars().collect::<Vec<char>>()
            ),
            None
        );
    }
}
