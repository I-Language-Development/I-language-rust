//! The CLI of the lexer.
// I Language lexer executable.
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

use std::{self, io::Write};

use I_Language_lexer::lex;

use tools::beta;
use tools::panic_handler;

use clap::Parser;


///////////////////
// CLI Arguments //
///////////////////

#[derive(Parser)]
#[command(author, version, about)]
struct Arguments {
    /// Weather to use experimental beta features or not
    #[arg(short = 'b', long = "beta")]
    beta: bool,

    /// The file to lex. If not provided, a REPL like input will be used
    file: Option<String>,
}


///////////////////
// MAIN FUNCTION //
///////////////////

fn main() {
    panic_handler::setup_handler();

    let arguments: Arguments = Arguments::parse();
    unsafe {
        beta::BETA_FLAG = arguments.beta;
    }

    let mut input: String = String::new();
    let source: &str;

    if let Some(file_name) = arguments.file.as_deref() {
        let file = std::fs::File::open(file_name).unwrap();
        let reader = std::io::BufReader::new(file);

        input = std::io::read_to_string(reader).unwrap();
        source = file_name;
    } else {
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        source = "<stdin>";
    }

    let start = std::time::Instant::now();
    println!("{:#?}", lex::lex(input.trim(), source));
    println!("Took {}ms", start.elapsed().as_millis());
}
