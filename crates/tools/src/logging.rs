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

#![allow(rustdoc::private_intra_doc_links)]

/////////////
// IMPORTS //
/////////////

use core::sync::atomic::{AtomicBool, Ordering};
use std::sync::{LazyLock, Mutex};

use env_logger::Builder;
use log::{set_boxed_logger, set_max_level, LevelFilter, Log, Metadata, Record};

#[cfg(feature = "cli")]
use clap_verbosity_flag::LogLevel;
#[cfg(feature = "cli")]
use log::Level;


///////////////
// CONSTANTS //
///////////////

/// Buffer of all log messages, even if they were disabled in the CLI.
pub static BUFFER: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

/// Whether or not the logger already has been initialized.
pub static INITIALIZED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));


////////////
// LOGGER //
////////////

/// Custom logger with a buffer.
#[derive(Debug)]
pub struct Logger(pub env_logger::Logger);

impl Log for Logger {
    /// Whether the logger is enabled for this record or not.
    ///
    /// # Parameters
    ///
    /// - `_`: The metadata of the current log record. As it's not used, the name is `_`.
    ///
    /// # Returns
    ///
    /// Whether the logger is enabled for this record or not.
    /// Will always return `true`, because the logger is enabled for every log
    /// record, to write it to the buffer.
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// # use log::Log;
    /// # use env_logger;
    /// # use tools::logging;
    /// # let logger: logging::Logger = logging::Logger(env_logger::Builder::new().build());
    /// # let metadata: log::Metadata = log::MetadataBuilder::new().build();
    /// assert!(logger.enabled(&metadata));
    ///
    /// ```
    ///
    /// # See also
    ///
    /// - [`Logger`]
    /// - [`Metadata`]
    #[inline(always)]
    fn enabled(&self, _: &Metadata) -> bool {
        // Level can't be under trace, so just return true
        true
    }

    /// Logs the specified log record to both stdout and the log message buffer.
    ///
    /// # Parameters
    ///
    /// - `record`: The log record to log.
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// # use log::Log;
    /// # use env_logger;
    /// # use tools::logging;
    /// # let logger: logging::Logger = logging::Logger(env_logger::Builder::new().build());
    /// # let record: log::Record = log::RecordBuilder::new().build();
    /// logger.log(&record);
    ///
    /// ```
    ///
    /// # See also
    ///
    /// - [`Logger`]
    /// - [`Record`]
    /// - [`BUFFER`]
    #[inline]
    #[allow(clippy::unwrap_used)] // Using `.unwrap()` because providing an Error is not possible and panicking would not make sense.
    fn log(&self, record: &Record) {
        if self.0.enabled(record.metadata()) {
            self.0.log(&record.clone());
        }

        BUFFER.lock().unwrap().push(format!(
            "[{} {} {}] {}",
            chrono::offset::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            record.level(),
            record.target(),
            record.args()
        ));
    }

    /// Flushes the inner logger.
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// # use log::Log;
    /// # use env_logger;
    /// # use tools::logging;
    /// # let logger: logging::Logger = logging::Logger(env_logger::Builder::new().build());
    /// logger.flush();
    ///
    /// ```
    ///
    /// # See also
    ///
    /// - [`Logger`]
    #[inline(always)]
    fn flush(&self) {
        self.0.flush();
    }
}


///////////////////
// OFF LOG LEVEL //
///////////////////

/// A log level that enables no message to be logged.
/// Useful for applications that have other ways of displaying information than
/// logging and use logging just to show what the program is doing.
#[cfg(feature = "cli")]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OffLevel;

#[cfg(feature = "cli")]
impl LogLevel for OffLevel {
    /// Returns the log level for this level filter.
    ///
    /// # Returns
    ///
    /// The log level for this level filter.
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// # use tools::logging;
    /// # use clap_verbosity_flag::LogLevel as _;
    /// assert_eq!(logging::OffLevel::default(), None);
    ///
    /// ```
    ///
    /// # See also
    ///
    /// - [`OffLevel`]
    /// - [`Level`]
    #[inline(always)]
    fn default() -> Option<Level> {
        None
    }
}


///////////////////
// SETUP LOGGING //
///////////////////

/// Sets up logging globally.
///
/// Unlike [`set_boxed_logger`], this method can be called multiple times
/// without panicking, due to the check if [`INITIALIZED`] is `true` or not.
///
/// # Parameters
///
/// - `level`: The logging level to log with. Only applies to stdout, not the buffer.
///
/// # Panics
///
/// Panics if the logger could not be set using [`set_boxed_logger`].
///
/// # Examples
///
/// ```rust
///
/// # use log;
/// # use tools::logging;
/// # let level: log::LevelFilter = log::LevelFilter::Error;
/// logging::setup(level);
///
/// ```
///
/// # See also
///
/// - [`Logger`]
/// - [`LevelFilter`]
#[inline]
#[allow(clippy::unnecessary_safety_doc)]
pub fn setup(level: LevelFilter) {
    if !INITIALIZED.load(Ordering::Relaxed) {
        set_boxed_logger(Box::new(Logger(Builder::new().filter_level(level).build())))
            .expect("Logger could not be initialized.");
        set_max_level(LevelFilter::Trace);

        INITIALIZED.store(true, Ordering::Relaxed);
    }
}
