// I Language parser errors.
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

use std;

use pest::error;
use pest::error::Error;
use pest::RuleType;
use pest::Span;


////////////
// ERRORS //
////////////

pub fn syntax_error<R>(error: Error<R>, path: &str)
where
    R: RuleType,
{
    eprintln!("\x1b[31;1mError:\x1b[0m Invalid syntax");

    match error.variant {
        error::ErrorVariant::ParsingError { .. } => {
            // TODO
        }
        error::ErrorVariant::CustomError { .. } => {
            // TODO
        }
    };

    println!("{:?}", error); // For debugging
    eprintln!("{}", error.with_path(path));

    std::process::exit(1);
}


/// Creates a new missing EOI token error.
///
/// # Parameters
///
/// - `span`: The span of the last token received
/// - `path`: The file path where the error occurred
///
/// # Aborts
///
/// Exits with error code 1.
///
/// # Examples
///
/// ```should_panic
/// # use pest;
/// # use I_Language_Rust::Parser::errors::missing_eoi_error;
///
/// missing_eoi_error(pest::Span::new("//This for some reason does not have a EOI token", 47, 48).unwrap(), "file.il");
/// ```
///
/// # See also
///
/// - [`filter::filter_eoi`]
/// - [EOI Token](https://docs.rs/pest/latest/pest/index.html#special-rules)
pub fn missing_eoi_error(span: Span<'_>, path: &str) {
    let error: Error<()> = Error::new_from_span(
        error::ErrorVariant::CustomError {
            message: String::from("Expected EOI token at the end of input."),
        },
        span,
    )
    .with_path(path);

    eprintln!("\x1b[31;1mError:\x1b[0m Missing EOI Token");
    eprintln!("{}", error);
    eprintln!("");

    eprintln!(
        "This is most likely not your fault. Please report this error on our GitHub issue tracker:"
    );
    eprintln!("https://github.com/I-Language-Development/I-language-rust/issues");

    std::process::exit(2);
}
