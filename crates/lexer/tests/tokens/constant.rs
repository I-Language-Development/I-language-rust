// I Language lexer constants tests.
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
    fn test_constant() {
        use lexer::tokens::constant::Constant;
        use lexer::tokens::token::TypeDefinition;

        let location: Location = Location {
            file: "tests".to_owned(),
            line: 1,
            column: 1,
        };

        assert_eq!(
            Constant::get_token(location.clone(), &"true".chars().collect::<Vec<char>>()),
            Some(Token {
                location: location.clone(),
                content: "true".to_owned(),
                token_type: TokenType::TypeDefinition(TypeDefinition::Boolean(Constant::True)),
            })
        );
        assert_eq!(
            Constant::get_token(location.clone(), &"false".chars().collect::<Vec<char>>()),
            Some(Token {
                location: location.clone(),
                content: "false".to_owned(),
                token_type: TokenType::TypeDefinition(TypeDefinition::Boolean(Constant::False)),
            })
        );
        assert_eq!(
            Constant::get_token(location.clone(), &"None".chars().collect::<Vec<char>>()),
            Some(Token {
                location: location.clone(),
                content: "None".to_owned(),
                token_type: TokenType::TypeDefinition(TypeDefinition::None),
            })
        );
        assert_eq!(
            Constant::get_token(location.clone(), &"/".chars().collect::<Vec<char>>()),
            None
        );
    }

    #[test]
    fn test_type() {
        use lexer::tokens::constant::Type;

        let location: Location = Location {
            file: "tests".to_owned(),
            line: 1,
            column: 1,
        };

        assert_eq!(
            Type::get_token(location.clone(), &"str".chars().collect::<Vec<char>>()),
            Some(Token {
                location: location.clone(),
                content: "string".to_owned(),
                token_type: TokenType::Type(Type::Str),
            })
        );
        assert_eq!(
            Type::get_token(location.clone(), &"string".chars().collect::<Vec<char>>()),
            Some(Token {
                location: location.clone(),
                content: "string".to_owned(),
                token_type: TokenType::Type(Type::Str),
            },)
        );
        assert_eq!(
            Type::get_token(location.clone(), &"int".chars().collect::<Vec<char>>()),
            Some(Token {
                location: location.clone(),
                content: "integer".to_owned(),
                token_type: TokenType::Type(Type::Int),
            },)
        );
        assert_eq!(
            Type::get_token(location.clone(), &"integer".chars().collect::<Vec<char>>()),
            Some(Token {
                location: location.clone(),
                content: "integer".to_owned(),
                token_type: TokenType::Type(Type::Int),
            },)
        );
        assert_eq!(
            Type::get_token(location.clone(), &"bool".chars().collect::<Vec<char>>()),
            Some(Token {
                location: location.clone(),
                content: "boolean".to_owned(),
                token_type: TokenType::Type(Type::Boolean),
            },)
        );
        assert_eq!(
            Type::get_token(location.clone(), &"boolean".chars().collect::<Vec<char>>()),
            Some(Token {
                location: location.clone(),
                content: "boolean".to_owned(),
                token_type: TokenType::Type(Type::Boolean),
            },)
        );
        assert_eq!(
            Type::get_token(location.clone(), &"/".chars().collect::<Vec<char>>()),
            None
        );
    }
}
