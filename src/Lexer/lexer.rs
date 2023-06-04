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

////////////////////////////////
// IMPORTS AND USE STATEMENTS //
////////////////////////////////

use std;

///////////////
// CONSTANTS //
///////////////

//const DIGITS_AS_STRINGS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
//const WHITE_SPACES: [char; 3] = [' ', '\t', '\n'];

//const ESCAPE_CHARACTERS: [char; 5] = ['"', '\\', 'n', 't', 'r']; // Does "'" not count?

// Version of the lexer
const VERSION: &str = "0.1.1";

//////////////////
// LEXER TOKENS //
//////////////////

#[derive(Debug)]
enum Mark {
    // Put it all into mark or create a operator enum?
    Equal,   // Equality
    Greater, // GreaterThan
    Less,    // LessThan
    NotEqual,
    LessEqual,
    GreaterEqual,
    Increase,
    Decrease,
    And,
    Or,
    Bang, // Use case?
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
    QuestionMark, // Ternary operator?
    Add,
    AddAssign,
    Subtract,
    SubtractAssign,
    Multiply,
    MultiplyAssign,
    Divide,
    DivideAssign,
    Modulo,
    ModuloAssign, // Does it return an array?
    BitOr,
    BitOrAssign, // Does it return an array?
    BitAnd,
    BitAndAssign, // Does it return an array?
    BitXor,
    BitXorAssign, // Does it return the true value? (Would make sense)
    ShiftLeft,
    ShiftLeftAssign, // ???
    ShiftRight,
    ShiftRightAssign, // ???
}

#[derive(Debug)]
enum Keyword {
    Class,
    Function, // Re: `(func[tion]|TYPE)`?
    Use,
    Import,
    If, // TODO (ElBe): Add elseif
    Else,
    Match,
    Case,
    Default, // Seems to be already in use by rust -> _Default? Also, what should this be?
    For,     // TODO (ElBe): Add in
    Return,
    Delete, // Rename to del or remove? Can stay this way though.
    Break,
    Continue,
    Try,
    Catch,
    Throw,
    Finally,
    While,
}

#[derive(Debug)]
enum BaseType {
    Integer,
    Float,
    Boolean,
    Char,
}

#[derive(Debug)]
enum CompoundType {
    //Array(BaseType),
    //DynamicArray(BaseType),
    //List(BaseType),
    String,
    //HashTable(BaseType),
    //HashMap(BaseType, BaseType),
    //Tuple, // Replaceable by array
    // Dict? Would be cool the way JS has it right now
}

#[derive(Debug)]
enum TokenType {
    BaseType(BaseType),
    Keyword(Keyword),
    CompoundType(CompoundType),
    Mark(Mark),
    Identifier,
}

#[derive(Debug)]
struct Token {
    _type: TokenType,
    value: Option<String>,
}

mod lexer_errors {
    // https://doc.rust-lang.org/std/error/trait.Error.html ... TODO (Ranastra)
    // TODO (Ranastra): Improve visuals of errors and put them in their own file

    pub fn unwrap<T>(result: Result<T, LexerError>) -> T {
        match result {
            Ok(value) => value,
            Err(err) => {
                println!("{:?}", err);
                std::process::exit(1);
            }
        }
    }

    enum LexerErrorType {
        StringNotClosed,
        CharNotClosed,
        UnknownEscapeSequence(char),
        MultilineCommentNotClosed,
        UnexpectedCharacter(char),
    }

    pub struct LexerError {
        _error: LexerErrorType,
        position: super::Position,
    }
    impl std::fmt::Debug for LexerError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Lexer error at {}\n", self.position)?; // Error number, traceback, is column included?

            match self._error {
                LexerErrorType::StringNotClosed => write!(f, "string not closed"),
                LexerErrorType::CharNotClosed => write!(f, "char not closed or to many chars"),
                LexerErrorType::UnknownEscapeSequence(c) => {
                    write!(f, "unknown escape sequenz: \"\\{}\"", c)
                }
                LexerErrorType::MultilineCommentNotClosed => {
                    write!(f, "multiline comment not closed")
                }
                LexerErrorType::UnexpectedCharacter(c) => {
                    write!(f, "unexpected characeter: '{}'", c)
                }
            }
        }
    }

    impl LexerError {
        pub fn string_not_closed(position: super::Position) -> LexerError {
            LexerError {
                _error: LexerErrorType::StringNotClosed,
                position,
            }
        }
        pub fn char_not_closed(position: super::Position) -> LexerError {
            LexerError {
                _error: LexerErrorType::CharNotClosed,
                position,
            }
        }
        pub fn unknown_escape_sequence(position: super::Position, character: char) -> LexerError {
            LexerError {
                _error: LexerErrorType::UnknownEscapeSequence(character),
                position,
            }
        }
        pub fn multiline_comment_not_closed(position: super::Position) -> LexerError {
            LexerError {
                _error: LexerErrorType::MultilineCommentNotClosed,
                position,
            }
        }
        pub fn unexpected_character(position: super::Position, character: char) -> LexerError {
            LexerError {
                _error: LexerErrorType::UnexpectedCharacter(character),
                position,
            }
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
    fn new_identifier(name: String) -> Token {
        Token {
            _type: TokenType::Identifier,
            value: Some(name),
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
    pub fn remove_single_line_comment(chars: &mut std::str::Chars) {
        let mut character: Option<char>;
        loop {
            character = chars.next();
            if let Some(c) = character {
                if c == '\n' {
                    // TODO (ElBe): Find better variable names
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
        start_position: &super::Position,
    ) -> Result<(), super::lexer_errors::LexerError> {
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
                            *column_number = 0;
                        }
                    } else {
                        return Err(
                            super::lexer_errors::LexerError::multiline_comment_not_closed(
                                *start_position,
                            ),
                        );
                    }
                } else if c == '\n' {
                    *line_number += 1;
                    *column_number = 0;
                }
            } else {
                return Err(
                    super::lexer_errors::LexerError::multiline_comment_not_closed(*start_position),
                );
            }
        }
    }
}

impl BaseType {
    fn get_character(
        chars: &mut std::str::Chars,
        column_number: &mut usize,
        start_position: Position,
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
                        // TODO (ElBe): \0x..., \0b..., \033[..., \u..., etc.
                        'n' => result = '\n',
                        't' => result = '\t',
                        'r' => result = '\r',
                        '\\' => result = '\\',
                        '\'' => result = '\'',
                        _ => {
                            return Err(lexer_errors::LexerError::unknown_escape_sequence(
                                start_position,
                                c,
                            ));
                        }
                    }
                } else {
                    return Err(lexer_errors::LexerError::char_not_closed(start_position));
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
                    Err(lexer_errors::LexerError::char_not_closed(start_position))
                }
            } else {
                Err(lexer_errors::LexerError::char_not_closed(start_position))
            }
        } else {
            Err(lexer_errors::LexerError::char_not_closed(start_position))
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
        start_position: Position,
    ) -> Result<String, lexer_errors::LexerError> {
        let mut string = String::new();
        let mut character;
        loop {
            character = chars.next();

            if let Some(c) = character {
                if c == '\\' {
                    character = chars.next();
                    *column_number += 1;
                    if let Some(c) = character {
                        match c {
                            'n' => string.push('\n'),
                            't' => string.push('\t'),
                            'r' => string.push('\r'),
                            '\\' => string.push('\\'),
                            '"' => string.push('"'),
                            _ => {
                                return Err(lexer_errors::LexerError::unknown_escape_sequence(
                                    start_position,
                                    c,
                                ));
                            }
                        }
                    } else {
                        return Err(lexer_errors::LexerError::string_not_closed(start_position));
                    }
                } else if c == '"' {
                    *line_number += 1;
                    break;
                } else {
                    if c == '\n' {
                        return Err(lexer_errors::LexerError::string_not_closed(start_position));
                    }
                    string.push(c);
                }
            } else {
                return Err(lexer_errors::LexerError::string_not_closed(start_position));
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

    fn identify_keyword(word: &String) -> Option<Keyword> {
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
    println!("\x1b[93mError:\x1b[0m"); // Red
    println!("    Currently the CLI does not work.");
    println!("");
    println!("Usage: lexer.py [PATH] [-h] [-v] [--types] [--values] [--no-split]");
    println!("Lexer of the I-programming language.");
    println!("Options:");
    println!("    -h, --help             Shows this help and exits.");
    println!("    -v, --version          Shows the version of the lexer and exits.");
    println!("    --types                Only print the types of the tokens.");
    println!("                           Incompatible with: --values");
    println!("    --values               Only print the values of the tokens.");
    println!("                           Incompatible with: --types");
    println!("    --no-split             Prints the tokens in a list, instead of line");
    println!("                           by line.");
}

fn open_file(path: &str) -> String {
    let text: String = std::fs::read_to_string(path).expect(&format!("File not found {}", path));
    text
}

fn match_cli_argument() -> (LexOptions, String) {
    let mut lex_options = LexOptions {
        types: false,
        values: false,
        no_split: false, // Currently the default
    };
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_help_message();
        std::process::exit(0);
    }
    let mut arg;
    for ind in 2..args.len() {
        arg = args[ind].to_lowercase();
        if arg == "-h" || arg == "--help" {
            print_help_message();
            std::process::exit(0);
        } else if arg == "-v" || arg == "--version" {
            println!("Version: {}", VERSION);
            std::process::exit(0);
        } else if arg == "--types" && !lex_options.values {
            lex_options.types = true;
        } else if arg == "--values" && !lex_options.types {
            lex_options.values = true;
        } else if arg == "--no-split" {
            lex_options.no_split = true;
        } else {
            println!("\x1b[93mError:\x1b[0m");
            println!("    Unknown argument: {}", arg);
            println!("");
            print_help_message();
            std::process::exit(1); // TODO (ElBe): Find exit codes
        }
    }
    (lex_options, args[1].clone())
}

#[derive(Clone, Copy)]
pub struct Position {
    line_number: usize,
    column_number: usize,
}

impl Position {
    fn new(line_number: usize, column_number: usize) -> Position {
        Position {
            line_number,
            column_number,
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "line: {}, column: {}",
            self.line_number, self.column_number
        )
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}|{})", self.line_number, self.column_number)
    }
}

fn lex(text: String) -> Vec<(Position, Token)> {
    // Bad name?
    let mut result: Vec<(Position, Token)> = Vec::new();
    let (mut line_number, mut column_number) = (1, 0); // Start with line 1?
    let mut character;
    let mut chars = text.chars();
    let mut last_char = None;
    let mut current_position: Position;
    loop {
        if let Some(c) = last_char {
            character = Some(c);
            last_char = None;
        } else {
            character = chars.next();
            column_number += 1;
        }
        current_position = Position::new(line_number, column_number);
        if let Some(c) = character {
            if c == '\n' {
                line_number += 1;
                column_number = 0;
            } else if c.is_whitespace() {
                continue;
            } else if c.is_alphabetic() || c == '_' {
                let word = Keyword::get_word(&mut chars, c, &mut last_char, &mut column_number);
                if word == "true" || word == "false" {
                    result.push((
                        current_position,
                        Token::new_base_type(BaseType::Boolean, word),
                    ));
                } else {
                    if let Some(keyword) = Keyword::identify_keyword(&word) {
                        result.push((current_position, Token::new_keyword(keyword)));
                    } else {
                        result.push((current_position, Token::new_identifier(word)));
                    }
                }
            } else if c == '{' {
                result.push((current_position, Token::new_mark(Mark::BraceOpen)));
            } else if c == '}' {
                result.push((current_position, Token::new_mark(Mark::BraceClose)));
            } else if c.is_numeric() {
                let (num, num_type) =
                    BaseType::get_number(&mut chars, c, &mut last_char, &mut column_number);
                result.push((current_position, Token::new_base_type(num_type, num)));
            } else if c == '=' {
                // variants =, ==
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '=' {
                        result.push((current_position, Token::new_mark(Mark::Equal)));
                    } else {
                        last_char = Some(c);
                        result.push((current_position, Token::new_mark(Mark::Set)));
                    }
                } else {
                    result.push((current_position, Token::new_mark(Mark::Set)));
                }
            } else if c == '\'' {
                let val = lexer_errors::unwrap(BaseType::get_character(
                    &mut chars,
                    &mut column_number,
                    current_position,
                ));
                result.push((current_position, Token::new_base_type(BaseType::Char, val)));
            } else if c == '"' {
                let val = lexer_errors::unwrap(CompoundType::get_string(
                    &mut chars,
                    &mut line_number,
                    &mut column_number,
                    current_position,
                ));
                result.push((
                    current_position,
                    Token::new_compound_type(CompoundType::String, val),
                ));
            } else if c == '/' {
                // variants //, /*, /, /=
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '/' {
                        comment::remove_single_line_comment(&mut chars);
                        line_number += 1;
                        column_number = 0;
                    } else if c == '*' {
                        lexer_errors::unwrap(comment::remove_multi_line_comment(
                            &mut chars,
                            &mut line_number,
                            &mut column_number,
                            &current_position,
                        ));
                    } else if c == '=' {
                        result.push((current_position, Token::new_mark(Mark::DivideAssign)));
                    } else {
                        result.push((current_position, Token::new_mark(Mark::Divide)));
                        last_char = Some(c);
                    }
                } else {
                    result.push((current_position, Token::new_mark(Mark::Divide)));
                    break;
                }
            } else if c == ';' {
                result.push((current_position, Token::new_mark(Mark::Semicolon)));
            } else if c == '[' {
                result.push((current_position, Token::new_mark(Mark::BracketOpen)));
            } else if c == ']' {
                result.push((current_position, Token::new_mark(Mark::BracketClose)));
            } else if c == '(' {
                result.push((current_position, Token::new_mark(Mark::ParenthesisOpen)));
            } else if c == ')' {
                result.push((current_position, Token::new_mark(Mark::ParenthesisClose)));
            } else if c == '.' {
                result.push((current_position, Token::new_mark(Mark::Dot)));
            } else if c == ',' {
                result.push((current_position, Token::new_mark(Mark::Comma)));
            } else if c == '?' {
                result.push((current_position, Token::new_mark(Mark::QuestionMark)));
            } else if c == ':' {
                result.push((current_position, Token::new_mark(Mark::Colon)));
            } else if c == '%' {
                // variants %, %=
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '=' {
                        result.push((current_position, Token::new_mark(Mark::ModuloAssign)));
                    } else {
                        last_char = Some(c);
                        result.push((current_position, Token::new_mark(Mark::Modulo)));
                    }
                } else {
                    result.push((current_position, Token::new_mark(Mark::Modulo)));
                }
            } else if c == '*' {
                // variants *, *=
                // TODO (ElBe): Add **
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '=' {
                        result.push((current_position, Token::new_mark(Mark::MultiplyAssign)));
                    } else {
                        last_char = Some(c);
                        result.push((current_position, Token::new_mark(Mark::Multiply)));
                    }
                } else {
                    result.push((current_position, Token::new_mark(Mark::Multiply)));
                }
            } else if c == '|' {
                // variants |, |=, ||
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '|' {
                        result.push((current_position, Token::new_mark(Mark::Or)));
                    } else if c == '=' {
                        result.push((current_position, Token::new_mark(Mark::BitOrAssign)));
                    } else {
                        last_char = Some(c);
                        result.push((current_position, Token::new_mark(Mark::BitOr)));
                    }
                } else {
                    result.push((current_position, Token::new_mark(Mark::BitOr)));
                }
            } else if c == '&' {
                // variants &, &=, &&
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '&' {
                        result.push((current_position, Token::new_mark(Mark::And)));
                    } else if c == '=' {
                        result.push((current_position, Token::new_mark(Mark::BitAndAssign)));
                    } else {
                        last_char = Some(c);
                        result.push((current_position, Token::new_mark(Mark::BitAnd)));
                    }
                } else {
                    result.push((current_position, Token::new_mark(Mark::BitAnd)));
                }
            } else if c == '+' {
                // variants +, ++, +=
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '+' {
                        result.push((current_position, Token::new_mark(Mark::Increase)));
                    } else if c == '=' {
                        result.push((current_position, Token::new_mark(Mark::AddAssign)));
                    } else {
                        last_char = Some(c);
                        result.push((current_position, Token::new_mark(Mark::Add)));
                    }
                } else {
                    result.push((current_position, Token::new_mark(Mark::Add)));
                }
            } else if c == '-' {
                // variants -, --, -=
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '-' {
                        result.push((current_position, Token::new_mark(Mark::Decrease)));
                    } else if c == '=' {
                        result.push((current_position, Token::new_mark(Mark::SubtractAssign)));
                    } else {
                        last_char = Some(c);
                        result.push((current_position, Token::new_mark(Mark::Subtract)));
                    }
                } else {
                    result.push((current_position, Token::new_mark(Mark::Subtract)));
                }
            } else if c == '>' {
                // variants >, >=, >>, >>=
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '=' {
                        result.push((current_position, Token::new_mark(Mark::GreaterEqual)));
                    } else if c == '>' {
                        character = chars.next();
                        column_number += 1;
                        if let Some(c) = character {
                            if c == '=' {
                                result.push((
                                    current_position,
                                    Token::new_mark(Mark::ShiftRightAssign),
                                ));
                            } else {
                                last_char = Some(c);
                                result.push((current_position, Token::new_mark(Mark::ShiftRight)));
                            }
                        } else {
                            result.push((current_position, Token::new_mark(Mark::ShiftRight)));
                        }
                    } else {
                        last_char = Some(c);
                        result.push((current_position, Token::new_mark(Mark::Greater)));
                    }
                } else {
                    result.push((current_position, Token::new_mark(Mark::Greater)));
                }
            } else if c == '<' {
                // variants <, <=, <<, <<=
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '=' {
                        result.push((current_position, Token::new_mark(Mark::LessEqual)));
                    } else if c == '<' {
                        character = chars.next();
                        column_number += 1;
                        if let Some(c) = character {
                            if c == '=' {
                                result.push((
                                    current_position,
                                    Token::new_mark(Mark::ShiftLeftAssign),
                                ));
                            } else {
                                last_char = Some(c);
                                result.push((current_position, Token::new_mark(Mark::ShiftLeft)));
                            }
                        } else {
                            result.push((current_position, Token::new_mark(Mark::ShiftLeft)));
                        }
                    } else {
                        last_char = Some(c);
                        result.push((current_position, Token::new_mark(Mark::Less)));
                    }
                } else {
                    result.push((current_position, Token::new_mark(Mark::Less)));
                }
            } else if c == '!' {
                // variants !, !=
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '=' {
                        result.push((current_position, Token::new_mark(Mark::NotEqual)));
                    } else {
                        last_char = Some(c);
                        result.push((current_position, Token::new_mark(Mark::Bang))); // Not, better name
                    }
                } else {
                    result.push((current_position, Token::new_mark(Mark::Bang)));
                }
            } else if c == '^' {
                // variants ^, ^=
                character = chars.next();
                column_number += 1;
                if let Some(c) = character {
                    if c == '=' {
                        result.push((current_position, Token::new_mark(Mark::BitXorAssign)));
                    } else {
                        last_char = Some(c);
                        result.push((current_position, Token::new_mark(Mark::BitXor)));
                    }
                } else {
                    result.push((current_position, Token::new_mark(Mark::BitXor)));
                }
            } else {
                lexer_errors::unwrap::<()>(Err(lexer_errors::LexerError::unexpected_character(
                    current_position,
                    c,
                )));
            }
        } else {
            break;
        }
    }
    result
}

fn main() {
    //let (lex_options, path) = match_cli_argument();
    //let text = open_file(&path);
    //let tokens = lex(text);
    let a: Result<(), lexer_errors::LexerError> = Err(lexer_errors::LexerError::string_not_closed(
        Position::new(2, 1),
    ));
    lexer_errors::unwrap(a);
    let string = "function test(){a= 4.5; a+=2; return a;}".to_string();

    let tokens_test = lex(string);
    println!("{:?}", tokens_test);
}
