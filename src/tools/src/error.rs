// I Language error handler.
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

//////////////
// IMPORTS //
/////////////

use std::fmt;


///////////
// ERROR //
///////////

/// Error object.
///
/// Use [`Error::new()`] to create error objects instead of using this struct.
///
/// # Parameters
///
/// - `name`: The errors name.
/// - `description`: The error description.
/// - `exit_code`: The errors exit code.
///
/// # Returns
///
/// A new `Error` object with the specified name and description.
///
/// # Examples
///
/// ```rust
/// # use localizer_rs;
/// localizer_rs::errors::Error {
///     name: "name".to_owned(),
///     description: "description".to_owned(),
///     exit_code: 1
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Error {
    /// The errors name.
    pub name: String,
    /// The error description.
    pub description: String,
    /// The errors exit code.
    pub exit_code: i32,
}

/// Display implementation for the error object.
impl fmt::Display for Error {
    /// Format implementation for the error object.
    ///
    /// # Parameters
    ///
    /// - `self`: The error object.
    /// - `f`: The [`fmt::Formatter`] to use.
    ///
    /// # Returns
    ///
    /// A [`fmt::Result`] containing the formatted error message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use localizer_rs;
    /// # let error = localizer_rs::errors::Error::new("name", "description", 1);
    /// println!("{}", error);
    /// ```
    ///
    /// # See also
    ///
    /// - [`fmt::Display`]
    /// - [`Error`]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1b[31;1m{}\x1b[0m: {}", self.name, self.description)
    }
}

impl Error {
    /// Creates a new error object.
    ///
    /// # Parameters
    ///
    /// - `name`: The errors name.
    /// - `description`: The error description.
    /// - `exit_code`: The errors exit code.
    ///
    /// # Returns
    ///
    /// A new `Error` object with the specified name and description.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use localizer_rs;
    /// localizer_rs::errors::Error::new("name", "description", 1);
    /// ```
    ///
    /// # See also
    ///
    /// - [`Error`]
    pub fn new(name: &str, description: &str, exit_code: i32) -> Error {
        return Error {
            name: name.to_owned(),
            description: description.to_owned(),
            exit_code: exit_code,
        };
    }

    /// Raises the error and exits with the specified exit code.
    ///
    /// # Parameters
    ///
    /// - `self`: The error object.
    /// - `details`: The error details.
    ///
    /// # Aborts
    ///
    /// Exits with the specified exit code.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// # use localizer_rs;
    /// # let error: localizer_rs::errors::Error = localizer_rs::errors::Error::new("name", "description", 1);
    /// error.raise("Something went very wrong");
    /// ```
    ///
    /// # See also
    ///
    /// - [`Error`]
    pub fn raise(&self, details: &str) {
        eprintln!("{}", self);
        eprintln!("{}", details);

        std::process::exit(self.exit_code);
    }

    /// Raises the error but does not exit the program.
    ///
    /// # Parameters
    ///
    /// - `self`: The error object.
    /// - `details`: The error details.
    ///
    /// # Examples
    ///
    /// ```
    /// # use localizer_rs;
    /// # let error: localizer_rs::errors::Error = localizer_rs::errors::Error::new("name", "description", 1);
    /// error.raise("Something went very wrong");
    /// ```
    ///
    /// # See also
    ///
    /// - [`Error`]
    pub fn raise_without_exit(&self, details: &str) {
        eprintln!("{}", self);
        eprintln!("{}", details);
    }
}
