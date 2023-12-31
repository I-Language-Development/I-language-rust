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

use lexer;
use tools;

use clap::Parser;
use clap_verbosity_flag::{Verbosity, WarnLevel};
use log::{debug, error, trace};


///////////////////
// CLI Arguments //
///////////////////

#[derive(Parser)]
#[command(author, version, about)]
struct CLI {
    /// The path of the program to run
    file: Option<String>,

    /// Changes the verbosity of the logger
    #[command(flatten)]
    verbosity: Verbosity<WarnLevel>,

    /// Weather to use experimental beta features or not
    #[arg(short, long)]
    beta: bool,

    /// The directory where the output should be written to
    #[arg(short = 'o', long = "output", default_value = "out/run")]
    output: String,
}


//////////////////
// MAIN PROGRAM //
//////////////////

fn main() {
    tools::panic_handler::setup_handler();

    let arguments: CLI = CLI::parse();
    unsafe {
        tools::beta::BETA_FLAG = arguments.beta;
    }

    tools::logging::setup_logging(arguments.verbosity.log_level_filter());

    let file_name: String;
    if let Some(custom_file_name) = arguments.file {
        file_name = custom_file_name;
    } else {
        if std::path::Path::new("src/main.il")
            .try_exists()
            .expect("Could not check whether `src/main.il` exists")
        {
            file_name = "src/main.il".to_owned();
        } else if std::path::Path::new("main.il")
            .try_exists()
            .expect("Could not check whether `main.il` exists")
        {
            file_name = "main.il".to_owned();
        } else {
            error!("Neither `src/main.il` nor `main.il` exist.");
            std::process::exit(1); // TODO (ElBe): Proper errors
        }
    }

    trace!("Attempting to open file `{file_name}`.");
    let _file: std::fs::File =
        std::fs::File::open(file_name.clone()).expect("File `{file_name}` could not be opened.");
    let reader: std::io::BufReader<std::fs::File> = std::io::BufReader::new(_file);

    trace!("Successfully opened file `{file_name}`.");
    let input: String = std::io::read_to_string(reader).unwrap();

    let start = std::time::Instant::now();
    let output = lexer::lex::lex(input.trim(), &file_name);
    debug!(
        "Lexing `{file_name}` took {}ms.",
        start.elapsed().as_millis()
    );

    let mut file = std::fs::File::create(arguments.output + "/tokens")
        .expect("Could not open file `{value}` for writing.");
    file.write_all(format!("{:#?}", output).as_bytes());
}
