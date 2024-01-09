// I Language logging tools.
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

/////////////
// IMPORTS //
/////////////

use env_logger;
use log;


///////////////////
// SETUP LOGGING //
///////////////////

/// Custom logger with a buffer.
pub struct Logger(env_logger::Logger);

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() >= log::Level::Trace
    }

    fn log(&self, record: &log::Record) {
        if self.0.enabled(record.metadata()) {
            self.0.log(&record.clone());
        }

        #[allow(unsafe_code)]
        unsafe {
            BUFFER.push(format!(
                "[{} {} {}] {}",
                chrono::offset::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                record.level(),
                record.target(),
                record.args()
            ));
        }
    }

    fn flush(&self) {
        self.0.flush();
    }
}

static mut BUFFER: Vec<String> = Vec::new();
static mut INITIALIZED: bool = false;

pub fn setup_logging(level: log::LevelFilter) {
    #[allow(unsafe_code)]
    unsafe {
        if !INITIALIZED {
            log::set_boxed_logger(Box::new(Logger(
                env_logger::Builder::new().filter_level(level).build(),
            )))
            .unwrap();
            log::set_max_level(log::LevelFilter::Trace);

            INITIALIZED = true;
        }
    }
}

pub fn buffer() -> &'static Vec<String> {
    #[allow(unsafe_code)]
    unsafe {
        &BUFFER
    }
}
