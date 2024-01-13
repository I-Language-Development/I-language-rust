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

#[cfg(feature = "cli")]
use clap_verbosity_flag::LogLevel;
use env_logger;
use log;


///////////////
// CONSTANTS //
///////////////

/// Buffer of all log messages, even if they were disabled in the CLI.
static mut BUFFER: Vec<String> = Vec::new();

/// Whether or not the logger already has been initialized.
static mut INITIALIZED: bool = false;

/// Returns a buffer of all log messages.
///
/// # Returns
///
/// A buffer of all log messages, even if they disabled in the CLI.
///
/// # Examples
///
/// ```rust
///
/// # use log::trace;
/// # use tools::logging;
/// # logging::setup(log::LevelFilter::Error);
/// trace!(target: "test", "My message!"); // This will not be send to stdout, but will be saved in the buffer
/// println!("{:?}", logging::buffer()); // ["[timestamp TRACE test] My message!"]
///
/// ```
///
/// # Safety
///
/// Uses an `unsafe` block to access the contents of [`BUFFER`].
///
/// # See also
///
/// - [`BUFFER`]
#[inline]
#[allow(clippy::unnecessary_safety_doc)]
pub fn buffer() -> &'static Vec<String> {
    // SAFETY: Uses an `unsafe` block to access the contents of `BUFFER`.
    #[allow(unsafe_code)]
    unsafe {
        &BUFFER
    }
}


////////////
// LOGGER //
////////////

/// Custom logger with a buffer.
#[derive(Debug)]
pub struct Logger(pub env_logger::Logger);

impl log::Log for Logger {
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
    /// - [`log::Metadata`]
    #[inline(always)]
    fn enabled(&self, _: &log::Metadata) -> bool {
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
    /// # Safety
    ///
    /// Uses an `unsafe` block to write to [`BUFFER`].
    ///
    /// # See also
    ///
    /// - [`Logger`]
    /// - [`log::Record`]
    /// - [`BUFFER`]
    #[inline]
    fn log(&self, record: &log::Record) {
        if self.0.enabled(record.metadata()) {
            self.0.log(&record.clone());
        }

        // SAFETY: Uses an `unsafe` block to write to `BUFFER`.
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
    /// assert_eq!(logging::OffLevel::default(), None);
    ///
    /// ```
    ///
    /// # See also
    ///
    /// - [`OffLevel`]
    /// - [`log::Level`]
    #[inline(always)]
    fn default() -> Option<log::Level> {
        None
    }
}


///////////////////
// SETUP LOGGING //
///////////////////

/// Sets up logging globally.
///
/// Unlike [`log::set_boxed_logger`], this method can be called multiple times
/// without panicking, due to the check if [`INITIALIZED`] is `true` or not.
///
/// # Parameters
///
/// - `level`: The logging level to log with. Only applies to stdout, not the buffer.
///
/// # Panics
///
/// Panics if the logger could not be set using [`log::set_boxed_logger`].
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
/// # Safety
///
/// Uses two `unsafe` blocks to read from and write to [`INITIALIZED`].
///
/// # See also
///
/// - [`Logger`]
/// - [`log::LevelFilter`]
#[inline]
#[allow(clippy::unnecessary_safety_doc)]
pub fn setup(level: log::LevelFilter) {
    let initialized: bool;

    // SAFETY: Uses an `unsafe` block to read from [`INITIALIZED`].
    #[allow(unsafe_code)]
    unsafe {
        initialized = INITIALIZED;
    };

    if !initialized {
        log::set_boxed_logger(Box::new(Logger(
            env_logger::Builder::new().filter_level(level).build(),
        )))
        .expect("Logger could not be initialized.");
        log::set_max_level(log::LevelFilter::Trace);

        // SAFETY: Uses an `unsafe` block to write to [`INITIALIZED`].
        #[allow(unsafe_code)]
        unsafe {
            INITIALIZED = true;
        };
    }
}
