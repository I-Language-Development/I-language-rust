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
    use lexer::tokens::mark::Mark;
    use lexer::tokens::token::{GetToken, Location, Token, TokenType};

    fn generate_test(location: &Location, input: &str, mark: Mark) -> bool {
        Mark::get_token(location.clone(), &input.chars().collect::<Vec<char>>())
            == Some(Token {
                location: location.clone(),
                content: input.to_owned(),
                token_type: TokenType::Mark(mark),
            })
    }

    #[test]
    fn test_display() {
        assert_eq!(&format!("{}", Mark::Add), "+");
        assert_eq!(&format!("{}", Mark::AddAssign), "+=");
        assert_eq!(&format!("{}", Mark::And), "&&");
        assert_eq!(&format!("{}", Mark::Assign), "=");
        assert_eq!(&format!("{}", Mark::At), "@");
        assert_eq!(&format!("{}", Mark::Bang), "!");
        assert_eq!(&format!("{}", Mark::BitAnd), "&");
        assert_eq!(&format!("{}", Mark::BitAndAssign), "&=");
        assert_eq!(&format!("{}", Mark::BitNot), "~");
        assert_eq!(&format!("{}", Mark::BitNotAssign), "~=");
        assert_eq!(&format!("{}", Mark::BitOr), "|");
        assert_eq!(&format!("{}", Mark::BitOrAssign), "|=");
        assert_eq!(&format!("{}", Mark::BitXor), "^");
        assert_eq!(&format!("{}", Mark::BitXorAssign), "^=");
        assert_eq!(&format!("{}", Mark::BraceOpen), "{");
        assert_eq!(&format!("{}", Mark::BraceClose), "}");
        assert_eq!(&format!("{}", Mark::BracketOpen), "[");
        assert_eq!(&format!("{}", Mark::BracketClose), "]");
        assert_eq!(&format!("{}", Mark::Colon), ":");
        assert_eq!(&format!("{}", Mark::Comma), ",");
        assert_eq!(&format!("{}", Mark::Decrease), "--");
        assert_eq!(&format!("{}", Mark::Divide), "/");
        assert_eq!(&format!("{}", Mark::DivideAssign), "/=");
        assert_eq!(&format!("{}", Mark::Dot), ".");
        assert_eq!(&format!("{}", Mark::Equal), "==");
        assert_eq!(&format!("{}", Mark::Exponentiation), "**");
        assert_eq!(&format!("{}", Mark::Greater), ">");
        assert_eq!(&format!("{}", Mark::GreaterEqual), ">=");
        assert_eq!(&format!("{}", Mark::Increase), "++");
        assert_eq!(&format!("{}", Mark::Less), "<");
        assert_eq!(&format!("{}", Mark::LessEqual), "<=");
        assert_eq!(&format!("{}", Mark::Modulo), "%");
        assert_eq!(&format!("{}", Mark::ModuloAssign), "%=");
        assert_eq!(&format!("{}", Mark::Multiply), "*");
        assert_eq!(&format!("{}", Mark::MultiplyAssign), "*=");
        assert_eq!(&format!("{}", Mark::NotEqual), "!=");
        assert_eq!(&format!("{}", Mark::Or), "||");
        assert_eq!(&format!("{}", Mark::ParenthesisOpen), "(");
        assert_eq!(&format!("{}", Mark::ParenthesisClose), ")");
        assert_eq!(&format!("{}", Mark::QuestionMark), "?");
        assert_eq!(&format!("{}", Mark::Range), "..");
        assert_eq!(&format!("{}", Mark::Semicolon), ";");
        assert_eq!(&format!("{}", Mark::ShiftLeft), "<<");
        assert_eq!(&format!("{}", Mark::ShiftLeftAssign), "<<=");
        assert_eq!(&format!("{}", Mark::ShiftRight), ">>");
        assert_eq!(&format!("{}", Mark::ShiftRightAssign), ">>=");
        assert_eq!(&format!("{}", Mark::Subtract), "-");
        assert_eq!(&format!("{}", Mark::SubtractAssign), "-=");
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_mark() {
        let location: Location = Location {
            file: "tests".to_owned(),
            line: 1,
            column: 1,
        };

        assert!(generate_test(&location, "+", Mark::Add));
        assert!(generate_test(&location, "+=", Mark::AddAssign));
        assert!(generate_test(&location, "&&", Mark::And));
        assert!(generate_test(&location, "=", Mark::Assign));
        assert!(generate_test(&location, "@", Mark::At));
        assert!(generate_test(&location, "!", Mark::Bang));
        assert!(generate_test(&location, "&", Mark::BitAnd));
        assert!(generate_test(&location, "&=", Mark::BitAndAssign));
        assert!(generate_test(&location, "~", Mark::BitNot));
        assert!(generate_test(&location, "~=", Mark::BitNotAssign));
        assert!(generate_test(&location, "|", Mark::BitOr));
        assert!(generate_test(&location, "|=", Mark::BitOrAssign));
        assert!(generate_test(&location, "^", Mark::BitXor));
        assert!(generate_test(&location, "^=", Mark::BitXorAssign));
        assert!(generate_test(&location, "{", Mark::BraceOpen));
        assert!(generate_test(&location, "}", Mark::BraceClose));
        assert!(generate_test(&location, "[", Mark::BracketOpen));
        assert!(generate_test(&location, "]", Mark::BracketClose));
        assert!(generate_test(&location, ":", Mark::Colon));
        assert!(generate_test(&location, ",", Mark::Comma));
        assert!(generate_test(&location, "--", Mark::Decrease));
        assert!(generate_test(&location, "/", Mark::Divide));
        assert!(generate_test(&location, "/=", Mark::DivideAssign));
        assert!(generate_test(&location, ".", Mark::Dot));
        assert!(generate_test(&location, "==", Mark::Equal));
        assert!(generate_test(&location, "**", Mark::Exponentiation));
        assert!(generate_test(&location, ">", Mark::Greater));
        assert!(generate_test(&location, ">=", Mark::GreaterEqual));
        assert!(generate_test(&location, "++", Mark::Increase));
        assert!(generate_test(&location, "<", Mark::Less));
        assert!(generate_test(&location, "<=", Mark::LessEqual));
        assert!(generate_test(&location, "%", Mark::Modulo));
        assert!(generate_test(&location, "%=", Mark::ModuloAssign));
        assert!(generate_test(&location, "*", Mark::Multiply));
        assert!(generate_test(&location, "*=", Mark::MultiplyAssign));
        assert!(generate_test(&location, "!=", Mark::NotEqual));
        assert!(generate_test(&location, "||", Mark::Or));
        assert!(generate_test(&location, "(", Mark::ParenthesisOpen));
        assert!(generate_test(&location, ")", Mark::ParenthesisClose));
        assert!(generate_test(&location, "?", Mark::QuestionMark));
        assert!(generate_test(&location, "..", Mark::Range));
        assert!(generate_test(&location, ";", Mark::Semicolon));
        assert!(generate_test(&location, "<<", Mark::ShiftLeft));
        assert!(generate_test(&location, "<<=", Mark::ShiftLeftAssign));
        assert!(generate_test(&location, ">>", Mark::ShiftRight));
        assert!(generate_test(&location, ">>=", Mark::ShiftRightAssign));
        assert!(generate_test(&location, "-", Mark::Subtract));
        assert!(generate_test(&location, "-=", Mark::SubtractAssign));
        assert_eq!(
            Mark::get_token(location.clone(), &" ".chars().collect::<Vec<char>>()),
            None
        );
    }
}
