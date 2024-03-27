// I Language terminal styling tools.
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

use regex::{Error, Regex};


////////////////////
// REMOVE STYLING //
////////////////////

// A trait for removing terminal styling (colors, formatting, etc.) from text.
// Used for using unstyled text in places where styled text can't be used.
pub trait RemoveStyling {
    /// Removes all terminal styling from the text.
    ///
    /// # Parameters
    ///
    /// - `self`: The text to remove the terminal styling in.
    ///
    /// # Returns
    ///
    /// A result of the text with the terminal styling removed or a regex compile error.
    ///
    /// # Errors
    ///
    /// Errors when the creation of the regex fails.
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// # use tools::terminal::RemoveStyling;
    /// assert_eq!(&(r"\x1B[31mRed\x1B[0m".remove_styling().unwrap()), "Red");
    ///
    /// ```
    fn remove_styling(&self) -> Result<String, Error>;
}

impl RemoveStyling for &str {
    #[inline]
    fn remove_styling(&self) -> Result<String, Error> {
        match Regex::new(r"\\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[mGK]") {
            Ok(regex) => Ok(regex.replace_all(self, "").to_string()),
            Err(error) => Err(error),
        }
    }
}
