// I Language parser token.
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

/////////////////
// TOKEN TYPES //
/////////////////

#[derive(Debug)]
pub enum Mark {
    Assignment,
    Constant,
    Variable,
    TypeDefinition,
    Not,
}

#[derive(Debug)]
pub enum Keyword {
    Class,
    Function,
    Use,
    Import,
    _From,
    As,
    If,
    Else,
    ElseIf,
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
    Finally,
    While,
    Variable,
    Constant,
    Not,
    Raise,
    Type,
    In,
}

#[derive(Debug)]
pub enum UseFeature {
    System,
    Shell,
    Settings,
    GarbageCollection,
    Rust,
    Speedups,
}

#[derive(Debug)]
pub enum TypeDefinition {
    List,
    String,
    Float,
    Integer,
    IntegerSign,
    StringContent,
    StringContentML,
}

#[derive(Debug)]
pub enum BaseType {
    Integer,
    Float,
    Boolean,
    Char,
    Any,
}

#[derive(Debug)]
pub enum CompoundType {
    Array(BaseType),
    List(BaseType), // Call it list?
    String,
    HashMap(BaseType, BaseType),
    Tuple(BaseType),
}

#[derive(Debug)]
pub enum TokenType {
    BaseType(BaseType),
    Keyword(Keyword),
    CompoundType(CompoundType),
    Mark(Mark),
    Identifier,

    UseFeature(UseFeature),
    TypeDefinition(TypeDefinition),
}

//////////////
// POSITION //
//////////////

#[derive(Debug)]
pub struct Position {
    pub line: usize,
    pub colon: usize,
}

///////////
// TOKEN //
///////////

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub content: String,
    pub inner: Option<Vec<Token>>,
    pub position: Position,
}
