// I Language main program.
// Version: 1.0.1

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

/////////////
// IMPORTS //
/////////////

use std::{self, io::Write};

use clap::Parser;
use clap_verbosity_flag::Verbosity;
use log::{debug, error, trace};


///////////////////
// CLI Arguments //
///////////////////

#[derive(Parser)]
#[command(author, version, about, long_version(format!("{} ({})", env!("CARGO_PKG_VERSION"), env!("GIT_HASH"))))]
struct Cli {
    /// The path of the program to run
    file: Option<String>,

    /// Changes the verbosity of the logger
    #[command(flatten)]
    verbosity: Verbosity<tools::logging::OffLevel>,

    /// Weather to use experimental beta features or not
    #[arg(short, long)]
    beta: bool,

    /// The directory where the output should be written to
    #[arg(short = 'o', long = "output", default_value = "out")]
    output: String,
}


//////////////////
// MAIN PROGRAM //
//////////////////

fn main() {
    tools::panic_handler::setup_handler();

    let arguments: Cli = Cli::parse();

    #[allow(unsafe_code)]
    unsafe {
        tools::beta::BETA_FLAG = arguments.beta;
    }

    tools::logging::setup(arguments.verbosity.log_level_filter());

    let file_name: String;
    if let Some(custom_file_name) = arguments.file {
        file_name = custom_file_name;
    } else if std::path::Path::new("src/main.il").exists() {
        file_name = "src/main.il".to_owned();
    } else if std::path::Path::new("main.il").exists() {
        file_name = "main.il".to_owned();
    } else {
        eprintln!("No file was specified and neither `src/mail.il` nor `main.il` exist.");
        error!("No file was specified and neither `src/main.il` nor `main.il` exist.");
        std::process::exit(1);
    }

    trace!("Attempting to open file `{file_name}`.");
    let _file: std::fs::File = std::fs::File::open(file_name.clone())
        .unwrap_or_else(|_| panic!("File `{file_name}` could not be opened."));
    let reader: std::io::BufReader<std::fs::File> = std::io::BufReader::new(_file);

    trace!("Successfully opened file `{file_name}`.");
    let input: String = std::io::read_to_string(reader).unwrap();

    let start: std::time::Instant = std::time::Instant::now();
    let output: Result<Vec<lexer::tokens::token::Token>, String> =
        lexer::lex::lex(input.trim(), &file_name);
    debug!(
        "Lexing `{file_name}` took {}ms.",
        start.elapsed().as_millis()
    );

    match output {
        Err(error) => {
            eprintln!("Compiling `{file_name}` was not successful:\n{error}");
            std::process::exit(1);
        }
        Ok(tokens) => {
            let mut file: std::fs::File =
                std::fs::File::create(String::new() + &arguments.output + "/tokens")
                    .unwrap_or_else(|_| {
                        panic!(
                            "Could not open file \"{}{}\" for writing.",
                            &arguments.output, "/tokens"
                        )
                    });
            file.write_all(format!("{:#?}", tokens).as_bytes())
                .unwrap_or_else(|_| {
                    panic!(
                        "Could not write to file \"{}{}\"",
                        &arguments.output, "/tokens"
                    )
                });
        }
    }
}
