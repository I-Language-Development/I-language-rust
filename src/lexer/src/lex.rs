// I Language lexer.
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

/////////////
// IMPORTS //
/////////////

use crate::tokens::constant::*;
use crate::tokens::keyword::*;
use crate::tokens::mark::*;
use crate::tokens::token::*;

use tools::error;
use tools::iterator::ConditionalPeeking;


//////////////////
// LEX FUNCTION //
//////////////////

pub fn lex(input: &str, file: &str) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];

    let mut single_quote: Option<usize> = None;
    let mut double_quote: Option<usize> = None;

    let mut chars: std::str::Chars;
    let mut last_character: char = '\0';

    let mut iterator: std::iter::Peekable<std::iter::Enumerate<std::str::Chars>>;
    let mut index: usize;

    let mut buffer: Vec<char>;
    let mut buffer_location: Option<Location> = None;

    let mut skips: usize = 0;

    for (mut line_number, line) in input.split("\n").enumerate() {
        line_number += 1;

        chars = line.chars();
        buffer = vec![];

        iterator = chars.into_iter().enumerate().peekable();

        while let Some((_index, character)) = iterator.next() {
            if skips > 0 {
                skips -= 1;
                last_character = character;
                continue;
            }

            if character.is_whitespace() && !(single_quote.is_some() || double_quote.is_some()) {
                last_character = character;
                continue;
            }

            index = _index + 1;
            let mut location: Location = Location {
                file: file.to_string(),
                line: line_number,
                column: index,
            };

            // TODO (ElBe): Add comments (/* */, //)
            if single_quote.is_some() || double_quote.is_some() {
                if index == line.len() {
                    let quote_index: usize = single_quote.or(double_quote).unwrap();
                    let quote_type: char = if single_quote.is_some() { '\'' } else { '"' };

                    let source: &str = &format!("   {line_number} | {line}");

                    if character == ';' {
                        let underline: &str = &format!(
                            "{0:>1$}{2}",
                            '^',
                            line_number.to_string().len() + quote_index + 6, /* < Amount of extra characters, in this case the pipe symbol and the five spaces*/
                            "^".repeat(index - quote_index)
                        );

                        error::Error::new(
                            "Error",
                            &format!("Unterminated string\n --> {location}"),
                            1,
                        )
                        .raise_without_exit(&format!(
                            "{source}\n{underline} = help: Add a `{quote_type}` here"
                        ));
                    }
                }

                match character {
                    '"' => {
                        if double_quote.is_some() && last_character != '\\' {
                            location.column = double_quote.unwrap();
                            result.push(Token {
                                location: location,
                                content: line[double_quote.unwrap()..index - 1].to_owned(),
                                token_type: TokenType::TypeDefinition(TypeDefinition::Str),
                            });
                            double_quote = None;
                        }
                    }
                    '\'' => {
                        if single_quote.is_some() && last_character != '\\' {
                            location.column = single_quote.unwrap();
                            result.push(Token {
                                location: location,
                                content: line[single_quote.unwrap()..index - 1].to_owned(),
                                token_type: TokenType::TypeDefinition(TypeDefinition::Str),
                            });
                            single_quote = None;
                        }
                    }
                    _ => {}
                }
            } else if character == '"' {
                double_quote = Some(index);
            } else if character == '\'' {
                single_quote = Some(index);
            } else if matches!(
                character,
                '+' | '-'
                    | '*'
                    | '/'
                    | '^'
                    | '%'
                    | '<'
                    | '>'
                    | '!'
                    | '='
                    | '&'
                    | '|'
                    | ':'
                    | '.'
                    | ','
                    | ';'
                    | '('
                    | ')'
                    | '{'
                    | '}'
                    | '['
                    | ']'
            ) {
                buffer.push(character);

                if let Some((_, next_character)) = iterator.clone().peek() {
                    if matches!(*next_character, '+' | '-' | '*' | '=' | '&' | '|' | '.') {
                        iterator.next();
                        buffer.push(*next_character);
                    /* Three character exceptions ("<<=", ">>=") */
                    } else if matches!(*next_character, '<' | '>') {
                        iterator.next();
                        buffer.push(*next_character);

                        if let Some((_, next_character)) = iterator.clone().peek() {
                            if next_character == &'=' {
                                iterator.next();
                                buffer.push('=');
                            }
                        }
                    }
                }

                if let Some((value, _skips)) = Mark::get_token(location.clone(), &buffer) {
                    result.push(value);
                    buffer.clear();
                }
            } else if character.is_digit(10) {
                buffer.push(character);
                buffer.append(
                    &mut iterator
                        .peek_while(|&(_, next_character)| {
                            next_character.is_digit(10) || next_character == '.'
                        })
                        .iter()
                        .map(|(_, found)| *found)
                        .collect::<Vec<char>>(),
                );

                if buffer.contains(&'.') {
                    if buffer
                        .iter()
                        .filter(|filter_character| **filter_character == '.')
                        .count()
                        > 1
                    {
                        // TODO (ElBe): Proper errors, filter ranges
                        eprintln!("Float with more than one dot");
                    }

                    result.push(Token {
                        location: location.clone(),
                        content: buffer.iter().collect::<String>(),
                        token_type: TokenType::TypeDefinition(TypeDefinition::Float),
                    });
                    buffer.clear();
                } else {
                    result.push(Token {
                        location: location.clone(),
                        content: buffer.iter().collect::<String>(),
                        token_type: TokenType::TypeDefinition(TypeDefinition::Integer),
                    });
                    buffer.clear();
                }
            } else if character.is_alphabetic() || character == '_' {
                buffer.push(character);
                buffer.append(
                    &mut iterator
                        .peek_while(|&(_, next_character)| {
                            next_character.is_alphabetic() || next_character == '_'
                        })
                        .iter()
                        .map(|(_, found)| *found)
                        .collect::<Vec<char>>(),
                );

                if buffer_location.is_none() {
                    buffer_location = Some(location.clone());
                }

                if let Some((value, _skips)) = Keyword::get_token(location.clone(), &buffer) {
                    result.push(value);
                    buffer.clear();
                } else if let Some((value, _skips)) = Type::get_token(location.clone(), &buffer) {
                    result.push(value);
                    buffer.clear();
                } else if let Some((value, _skips)) = Constant::get_token(location.clone(), &buffer)
                {
                    result.push(value);
                    buffer.clear();
                } else {
                    result.push(Token {
                        location: location.clone(),
                        content: buffer.iter().collect::<String>(),
                        token_type: TokenType::Identifier,
                    });

                    buffer.clear()
                }
            } else {
                let source: &str = &format!("   {line_number} | {line}");
                let underline: &str = &format!(
                    "{0:>1$}",
                    '^',
                    line_number.to_string().len() + index + 6 /* < Amount of extra characters, in this case the pipe symbol and the five spaces*/
                );
                error::Error::new(
                    "Error",
                    &format!("Syntax error\n --> {file}:{line_number}:{index}"),
                    2,
                )
                .raise(&format!(
                    "{source}\n{underline} = help: Unexpected character `{character}`"
                ));
            }

            last_character = character;
        }
    }

    return result;
}
