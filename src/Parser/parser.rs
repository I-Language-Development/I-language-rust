// I Language parser.
// Version: 0.1.1

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

use crate::Parser::errors;

use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;


///////////////////
// PARSER STRUCT //
///////////////////

#[cfg(not(feature = "beta"))]
#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("../Grammar/grammar.pest");
// Make sure grammar.pest is recompiled on every execution

#[cfg(feature = "beta")]
#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("../Grammar/grammar_beta.pest");
// Make sure grammar_beta.pest is recompiled on every execution

#[derive(Parser)]
#[cfg_attr(not(feature = "beta"), grammar = "Grammar/grammar.pest")]
#[cfg_attr(feature = "beta", grammar = "Grammar/grammar_beta.pest")]
pub struct IParser;


////////////////////
// PARSE FUNCTION //
////////////////////

pub fn parse<'a>(input: &'a str, path: &'a str) -> Option<Pairs<'a, Rule>> {
    let parse_result = IParser::parse(Rule::file, &input);

    match &parse_result {
        Ok(value) => {
            println!("{:#?}", value);
            return Some(value.to_owned());
        }
        Err(error) => errors::syntax_error(error.to_owned(), &path),
    };

    return None;
}
