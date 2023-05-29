// I Language lexer.
// Version: 0.1.0

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

use std;

//const DIGITS_AS_STRINGS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
//const WHITE_SPACES: [char; 3] = [' ', '\t', '\n'];

//const STRING_ESCAPE_CHARACTERS: [char; 5] = ['"', '\\', 'n', 't', 'r'];

const VERSION: &str = "0.1.0";

enum Mark {
    Equal,
    Greater,
    Less,
    NotEqual,
    LessEqual,
    GreaterEqual,
    Increase,
    Decrease,
    And,
    Or,
    Bang,
    Semicolon,
    Colon,
    Dot,
    Comma,
    Set,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    ParenthesisOpen,
    ParenthesisClose,
    QuestionMark,
    Add,
    AddAssign,
    Subtract,
    SubtractAssign,
    Multiply,
    MultiplyAssign,
    Divide,
    DivideAssign,
    Modulo,
    ModuloAssign,
    BitOr,
    BitOrAssign,
    BitAnd,
    BitAndAssign,
    BitXor,
    BitXorAssign,
    ShiftLeft,
    ShiftLeftAssign,
    ShiftRight,
    ShiftRightAssign,
}

enum Keyword {
    Class,
    Function,
    Use,
    Import,
    If,
    Else,
    Match,
    Case,
    Default,
    For,
    Return,
    Delete,
    Break,
    Continue,
    Try,
    Catch,
    Throw,
    Finally,
    While,
}

enum BaseType {
    Integer,
    Float,
    Boolean,
    Char,
}

enum CompoundType {
    //Array(BaseType),
    //DynamicArray(BaseType),
    String,
    //HashTable(BaseType),
    //HashMap(BaseType, BaseType),
    //Tuple,
}

enum TokenType {
    BaseType(BaseType),
    Keyword(Keyword),
    CompoundType(CompoundType),
    Mark(Mark),
    Identifier,
}

struct Token {
    _type: TokenType,
    value: Option<String>,
}

mod lexer_errors {
    // https://doc.rust-lang.org/std/error/trait.Error.html ... TODO
    // haha get that boring try catch!!

    pub fn unwrap<T>(result: Result<T, LexerError>) -> T {
        match result {
            Ok(value) => value,
            Err(err) => {
                println!("{:?}", err);
                std::process::exit(1);
            }
        }
    }

    enum _LexerError {
        StringNotClosed(StringNotClosed),
        CharNotClosed(CharNotClosed),
        UnknownEscapeSequence(UnknownEscapeSequence),
        MultilineCommentNotClosed(MultilineCommentNotClosed),
        UnexpectedCharacter(UnexpectedCharacter),
    }

    pub struct LexerError {
        _error: _LexerError,
        line_number: usize,
        column_number: usize,
    }
    impl std::fmt::Debug for LexerError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "lexer error at line {} collumn {}",
                self.line_number, self.column_number
            )?;
            match self._error {
                _LexerError::StringNotClosed(ref err) => write!(f, ": {}", err),
                _LexerError::CharNotClosed(ref err) => write!(f, ": {}", err),
                _LexerError::UnknownEscapeSequence(ref err) => write!(f, ": {}", err),
                _LexerError::MultilineCommentNotClosed(ref err) => write!(f, ": {}", err),
                _LexerError::UnexpectedCharacter(ref err) => write!(f, ": {}", err),
            }
        }
    }

    impl LexerError {
        pub fn string_not_closed(line_number: usize, column_number: usize) -> LexerError {
            LexerError {
                _error: _LexerError::StringNotClosed(StringNotClosed {}),
                line_number,
                column_number,
            }
        }
        pub fn char_not_closed(line_number: usize, column_number: usize) -> LexerError {
            LexerError {
                _error: _LexerError::CharNotClosed(CharNotClosed {}),
                line_number,
                column_number,
            }
        }
        pub fn unknown_escape_sequence(
            line_number: usize,
            column_number: usize,
            character: char,
        ) -> LexerError {
            LexerError {
                _error: _LexerError::UnknownEscapeSequence(UnknownEscapeSequence { character }),
                line_number,
                column_number,
            }
        }
        pub fn multiline_comment_not_closed(
            line_number: usize,
            column_number: usize,
        ) -> LexerError {
            LexerError {
                _error: _LexerError::MultilineCommentNotClosed(MultilineCommentNotClosed {}),
                line_number,
                column_number,
            }
        }
        pub fn unexpected_character(
            line_number: usize,
            column_number: usize,
            character: char,
        ) -> LexerError {
            LexerError {
                _error: _LexerError::UnexpectedCharacter(UnexpectedCharacter { character }),
                line_number,
                column_number,
            }
        }
    }

    struct UnknownEscapeSequence {
        character: char,
    }
    impl std::fmt::Display for UnknownEscapeSequence {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "unknown escape sequence: '\\{}'", self.character)
        }
    }

    struct CharNotClosed {}
    impl std::fmt::Display for CharNotClosed {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "char not closed or to many chars")
        }
    }

    struct StringNotClosed {}
    impl std::fmt::Display for StringNotClosed {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "string not closed")
        }
    }

    struct MultilineCommentNotClosed {}
    impl std::fmt::Display for MultilineCommentNotClosed {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "multiline comment not closed")
        }
    }

    struct UnexpectedCharacter {
        character: char,
    }
    impl std::fmt::Display for UnexpectedCharacter {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "unexpected character: '{}'", self.character)
        }
    }
}

impl Token {
    fn new_base_type(base_type: BaseType, value: String) -> Token {
        Token {
            _type: TokenType::BaseType(base_type),
            value: Some(value),
        }
    }
    fn new_identifier(value: String) -> Token {
        Token {
            _type: TokenType::Identifier,
            value: Some(value),
        }
    }
    fn new_keyword(keyword: Keyword) -> Token {
        Token {
            _type: TokenType::Keyword(keyword),
            value: None,
        }
    }
    fn new_compound_type(compound_type: CompoundType, value: String) -> Token {
        Token {
            _type: TokenType::CompoundType(compound_type),
            value: Some(value),
        }
    }
    fn new_mark(mark: Mark) -> Token {
        Token {
            _type: TokenType::Mark(mark),
            value: None,
        }
    }
}

mod comment {
    use super::*;
    pub fn remove_single_line_comment(chars: &mut std::str::Chars) {
        let mut character: Option<char>;
        loop {
            character = chars.next();
            if let Some(c) = character {
                if c == '\n' {
                    return;
                }
            } else {
                return;
            }
        }
    }
    pub fn remove_multi_line_comment(
        chars: &mut std::str::Chars,
        line_number: &mut usize,
        column_number: &mut usize,
    ) -> Result<(), lexer_errors::LexerError> {
        let start_position = (*line_number, *column_number);
        let mut character: Option<char>;
        loop {
            character = chars.next();
            *column_number += 1;
            if let Some(c) = character {
                if c == '*' {
                    character = chars.next();
                    *column_number += 1;
                    if let Some(c) = character {
                        if c == '/' {
                            return Ok(());
                        } else if c == '\n' {
                            *line_number += 1;
                            *column_number = 1;
                        }
                    } else {
                        return Err(lexer_errors::LexerError::multiline_comment_not_closed(
                            start_position.0,
                            start_position.1,
                        ));
                    }
                } else if c == '\n' {
                    *line_number += 1;
                    *column_number = 1;
                }
            } else {
                return Err(lexer_errors::LexerError::multiline_comment_not_closed(
                    start_position.0,
                    start_position.1,
                ));
            }
        }
    }
}

impl BaseType {
    fn get_character(
        chars: &mut std::str::Chars,
        line_number: usize,
        column_number: &mut usize,
    ) -> Result<String, lexer_errors::LexerError> {
        let mut character = chars.next();
        *column_number += 1;
        let result: char;
        if let Some(c) = character {
            if c == '\\' {
                character = chars.next();
                *column_number += 1;
                if let Some(c) = character {
                    match c {
                        'n' => result = '\n',
                        't' => result = '\t',
                        'r' => result = '\r',
                        '\\' => result = '\\',
                        '\'' => result = '\'',
                        _ => {
                            return Err(lexer_errors::LexerError::unknown_escape_sequence(
                                line_number,
                                *column_number,
                                c,
                            ));
                        }
                    }
                } else {
                    return Err(lexer_errors::LexerError::char_not_closed(
                        line_number,
                        *column_number,
                    ));
                }
            } else {
                result = c;
            }
            character = chars.next();
            *column_number += 1;
            if let Some(c) = character {
                if c == '\'' {
                    Ok(result.to_string())
                } else {
                    Err(lexer_errors::LexerError::char_not_closed(
                        line_number,
                        *column_number,
                    ))
                }
            } else {
                Err(lexer_errors::LexerError::char_not_closed(
                    line_number,
                    *column_number,
                ))
            }
        } else {
            Err(lexer_errors::LexerError::char_not_closed(
                line_number,
                *column_number,
            ))
        }
    }

    fn get_number(
        chars: &mut std::str::Chars,
        c: char,
        last_char: &mut Option<char>,
        column_number: &mut usize,
    ) -> (String, BaseType) {
        let mut number = String::new();
        number.push(c);
        let mut is_float = false;
        let mut character: Option<char>;
        loop {
            character = chars.next();
            *column_number += 1;
            if let Some(c) = character {
                if c.is_numeric() {
                    number.push(c);
                } else if c == '.' {
                    if !is_float {
                        is_float = true;
                        number.push(c);
                    } else {
                        *last_char = Some(c);
                        break;
                    }
                } else if c == '_' {
                    continue;
                } else {
                    *last_char = Some(c);
                    break;
                }
            } else {
                break;
            }
        }
        if is_float {
            (number, BaseType::Float)
        } else {
            (number, BaseType::Integer)
        }
    }
}

impl CompoundType {
    fn get_string(
        chars: &mut std::str::Chars,
        line_number: &mut usize,
        column_number: &mut usize,
    ) -> Result<String, lexer_errors::LexerError> {
        let mut string = String::new();
        let mut character: Option<char>;
        loop {
            character = chars.next();

            if let Some(c) = character {
                if c == '\\' {
                    character = chars.next();
                    *column_number += 2;
                    if let Some(c) = character {
                        match c {
                            'n' => string.push('\n'),
                            't' => string.push('\t'),
                            'r' => string.push('\r'),
                            '\\' => string.push('\\'),
                            '"' => string.push('"'),
                            _ => {
                                return Err(lexer_errors::LexerError::unknown_escape_sequence(
                                    *line_number,
                                    *column_number,
                                    c,
                                ));
                            }
                        }
                    } else {
                        return Err(lexer_errors::LexerError::string_not_closed(
                            *line_number,
                            *column_number,
                        ));
                    }
                } else if c == '"' {
                    *line_number += 1;
                    break;
                } else {
                    if c == '\n' {
                        return Err(lexer_errors::LexerError::string_not_closed(
                            *line_number,
                            *column_number,
                        ));
                    }
                    string.push(c);
                }
            } else {
                return Err(lexer_errors::LexerError::string_not_closed(
                    *line_number,
                    *column_number,
                ));
            }
        }
        Ok(string)
    }
}

impl Keyword {
    fn get_word(
        chars: &mut std::str::Chars,
        c: char,
        last_char: &mut Option<char>,
        column_number: &mut usize,
    ) -> String {
        let mut word = String::new();
        word.push(c);
        let mut character;
        loop {
            character = chars.next();
            *column_number += 1;
            if let Some(c) = character {
                if c.is_alphanumeric() || c == '_' {
                    word.push(c);
                } else {
                    *last_char = Some(c);
                    break;
                }
            } else {
                break;
            }
        }
        word
    }

    fn match_keyword(word: &String) -> Option<Keyword> {
        match word.as_str() {
            "else" => Some(Keyword::Else),
            "class" => Some(Keyword::Class),
            "function" => Some(Keyword::Function),
            "if" => Some(Keyword::If),
            "use" => Some(Keyword::Use),
            "import" => Some(Keyword::Import),
            "match" => Some(Keyword::Match),
            "case" => Some(Keyword::Case),
            "default" => Some(Keyword::Default),
            "for" => Some(Keyword::For),
            "while" => Some(Keyword::While),
            "return" => Some(Keyword::Return),
            "delete" => Some(Keyword::Delete),
            "break" => Some(Keyword::Break),
            "continue" => Some(Keyword::Continue),
            "try" => Some(Keyword::Try),
            "catch" => Some(Keyword::Catch),
            "throw" => Some(Keyword::Throw),
            "finally" => Some(Keyword::Finally),
            _ => None,
        }
    }
}

struct LexOptions {
    types: bool,
    values: bool,
    no_split: bool,
}

fn print_help_message() {
    println!("Usage: lexer.py [PATH] [-h] [-v] [--types] [--values] [--no-split]");
    println!("Lexer of the I-programming language.");
    println!("Options:");
    println!("    -h, --help             Shows this help and exits.");
    println!("    -v, --version          Shows the version of the lexer and exits.");
    println!("    --types                Only print the types of the tokens.");
    println!("    --values               Only print the values of the tokens.");
    println!("    --no-split             Prints the tokens in a list.");
}

fn open_file(path: &str) -> String {
    let text: String = std::fs::read_to_string(path).expect(&format!("File not found {}", path));
    text
}

fn match_cli_argument() -> (LexOptions, String) {
    let mut lex_options: LexOptions = LexOptions {
        types: false,
        values: false,
        no_split: false,
    };
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        // not my problem TODO
        print_help_message();
        std::process::exit(0);
    }
    let mut arg: String;
    for ind in 2..args.len() {
        arg = args[ind].to_lowercase();
        if arg == "-h" || arg == "--help" {
            print_help_message();
            std::process::exit(0);
        } else if arg == "-v" || arg == "--version" {
            println!("Version: {}", VERSION);
            std::process::exit(0);
        } else if arg == "--types" {
            lex_options.types = true;
        } else if arg == "--values" {
            lex_options.values = true;
        } else if arg == "--no-split" {
            lex_options.no_split = true;
        } else {
            println!("Unknown argument: {}", arg);
            std::process::exit(0);
        }
    }
    (lex_options, args[1].clone())
}

fn lex(text: String) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let (mut line_number, mut column_number) = (1, 1);
    let mut character: Option<char>;
    let mut chars = text.chars();
    let mut last_char: Option<char> = None;
    loop {
        if let Some(c) = last_char {
            character = Some(c);
        } else {
            character = chars.next();
            column_number += 1;
        }
        if let Some(c) = character {
            if c.is_whitespace() {
                continue;
            } else if c.is_alphabetic() {
                let word = Keyword::get_word(&mut chars, c, &mut last_char, &mut column_number);
                if word == "true" || word == "false" {
                    result.push(Token::new_base_type(BaseType::Boolean, word));
                } else {
                    if let Some(keyword) = Keyword::match_keyword(&word) {
                        result.push(Token::new_keyword(keyword));
                    } else {
                        result.push(Token::new_identifier(word));
                    }
                }
            } else if c == '\n' {
                line_number += 1;
                column_number = 1;
            } else if c == '{' {
                result.push(Token::new_mark(Mark::BraceOpen));
            } else if c == '}' {
                result.push(Token::new_mark(Mark::BraceClose));
            } else if c.is_numeric() {
                let (num, num_type) =
                    BaseType::get_number(&mut chars, c, &mut last_char, &mut column_number);
                result.push(Token::new_base_type(num_type, num));
            } else if c == '=' {
                // variants =, ==
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '=' {
                        result.push(Token::new_mark(Mark::Equal));
                    } else {
                        last_char = Some(c);
                        result.push(Token::new_mark(Mark::Set));
                    }
                } else {
                    result.push(Token::new_mark(Mark::Set));
                }
            } else if c == '\'' {
                let val = lexer_errors::unwrap(BaseType::get_character(
                    &mut chars,
                    line_number,
                    &mut column_number,
                ));
                result.push(Token::new_base_type(BaseType::Char, val));
            } else if c == '"' {
                let val = lexer_errors::unwrap(CompoundType::get_string(
                    &mut chars,
                    &mut line_number,
                    &mut column_number,
                ));
                result.push(Token::new_compound_type(CompoundType::String, val));
            } else if c == '/' {
                // variants //, /*, /, /=
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '/' {
                        comment::remove_single_line_comment(&mut chars);
                        line_number += 1;
                        column_number = 1;
                    } else if c == '*' {
                        lexer_errors::unwrap(comment::remove_multi_line_comment(
                            &mut chars,
                            &mut line_number,
                            &mut column_number,
                        ));
                    } else if c == '=' {
                        result.push(Token::new_mark(Mark::DivideAssign));
                    } else {
                        result.push(Token::new_mark(Mark::Divide));
                        last_char = Some(c);
                    }
                } else {
                    result.push(Token::new_mark(Mark::Divide));
                    break;
                }
            } else if c == ';' {
                result.push(Token::new_mark(Mark::Semicolon));
            } else if c == '[' {
                result.push(Token::new_mark(Mark::BracketOpen));
            } else if c == ']' {
                result.push(Token::new_mark(Mark::BracketClose));
            } else if c == '(' {
                result.push(Token::new_mark(Mark::ParenthesisOpen));
            } else if c == ')' {
                result.push(Token::new_mark(Mark::ParenthesisClose));
            } else if c == '.' {
                result.push(Token::new_mark(Mark::Dot));
            } else if c == ',' {
                result.push(Token::new_mark(Mark::Comma));
            } else if c == '?' {
                result.push(Token::new_mark(Mark::QuestionMark));
            } else if c == ':' {
                result.push(Token::new_mark(Mark::Colon));
            } else if c == '%' {
                // variants %, %=
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '=' {
                        result.push(Token::new_mark(Mark::ModuloAssign));
                    } else {
                        last_char = Some(c);
                        result.push(Token::new_mark(Mark::Modulo));
                    }
                } else {
                    result.push(Token::new_mark(Mark::Modulo));
                }
            } else if c == '*' {
                // variants *, *=
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '=' {
                        result.push(Token::new_mark(Mark::MultiplyAssign));
                    } else {
                        last_char = Some(c);
                        result.push(Token::new_mark(Mark::Multiply));
                    }
                } else {
                    result.push(Token::new_mark(Mark::Multiply));
                }
            } else if c == '|' {
                // variants |, |=, ||
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '|' {
                        result.push(Token::new_mark(Mark::Or));
                    } else if c == '=' {
                        result.push(Token::new_mark(Mark::BitOrAssign));
                    } else {
                        last_char = Some(c);
                        result.push(Token::new_mark(Mark::BitOr));
                    }
                } else {
                    result.push(Token::new_mark(Mark::BitOr));
                }
            } else if c == '&' {
                // variants &, &=, &&
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '&' {
                        result.push(Token::new_mark(Mark::And));
                    } else if c == '=' {
                        result.push(Token::new_mark(Mark::BitAndAssign));
                    } else {
                        last_char = Some(c);
                        result.push(Token::new_mark(Mark::BitAnd));
                    }
                } else {
                    result.push(Token::new_mark(Mark::BitAnd));
                }
            } else if c == '+' {
                // variants +, ++, +=
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '+' {
                        result.push(Token::new_mark(Mark::Increase));
                    } else if c == '=' {
                        result.push(Token::new_mark(Mark::AddAssign));
                    } else {
                        last_char = Some(c);
                        result.push(Token::new_mark(Mark::Add));
                    }
                } else {
                    result.push(Token::new_mark(Mark::Add));
                }
            } else if c == '-' {
                // variants -, --, -=
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '-' {
                        result.push(Token::new_mark(Mark::Decrease));
                    } else if c == '=' {
                        result.push(Token::new_mark(Mark::SubtractAssign));
                    } else {
                        last_char = Some(c);
                        result.push(Token::new_mark(Mark::Subtract));
                    }
                } else {
                    result.push(Token::new_mark(Mark::Subtract));
                }
            } else if c == '>' {
                // variants >, >=, >>, >>=
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '=' {
                        result.push(Token::new_mark(Mark::GreaterEqual));
                    } else if c == '>' {
                        character = chars.next();
                        column_number += 1;
                        if let Some(c) = character {
                            if c == '=' {
                                result.push(Token::new_mark(Mark::ShiftRightAssign));
                            } else {
                                last_char = Some(c);
                                result.push(Token::new_mark(Mark::ShiftRight));
                            }
                        } else {
                            result.push(Token::new_mark(Mark::ShiftRight));
                        }
                    } else {
                        last_char = Some(c);
                        result.push(Token::new_mark(Mark::Greater));
                    }
                } else {
                    result.push(Token::new_mark(Mark::Greater));
                }
            } else if c == '<' {
                // variants <, <=, <<, <<=
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '=' {
                        result.push(Token::new_mark(Mark::LessEqual));
                    } else if c == '<' {
                        character = chars.next();
                        column_number += 1;
                        if let Some(c) = character {
                            if c == '=' {
                                result.push(Token::new_mark(Mark::ShiftLeftAssign));
                            } else {
                                last_char = Some(c);
                                result.push(Token::new_mark(Mark::ShiftLeft));
                            }
                        } else {
                            result.push(Token::new_mark(Mark::ShiftLeft));
                        }
                    } else {
                        last_char = Some(c);
                        result.push(Token::new_mark(Mark::Less));
                    }
                } else {
                    result.push(Token::new_mark(Mark::Less));
                }
            } else if c == '!' {
                // variants !, !=
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '=' {
                        result.push(Token::new_mark(Mark::NotEqual));
                    } else {
                        last_char = Some(c);
                        result.push(Token::new_mark(Mark::Bang));
                    }
                } else {
                    result.push(Token::new_mark(Mark::Bang));
                }
            } else if c == '^' {
                // variants ^, ^=
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '=' {
                        result.push(Token::new_mark(Mark::BitXorAssign));
                    } else {
                        last_char = Some(c);
                        result.push(Token::new_mark(Mark::BitXor));
                    }
                } else {
                    result.push(Token::new_mark(Mark::BitXor));
                }
            } else {
                    lexer_errors::unwrap::<()>(Err(
                        lexer_errors::LexerError::unexpected_character(
                            line_number,
                            column_number,
                            c,
                        ),
                    ));
                
            }
        } else {
            break;
        }
    }
    result
}

fn main() {
    let (lex_options, path) = match_cli_argument();
    let text = open_file(&path);
    let tokens = lex(text);
    // let a: Result<(), lexer_errors::LexerError> =
    //     Err(lexer_errors::LexerError::string_not_closed(1, 1));
    // lexer_errors::unwrap(a);
}
