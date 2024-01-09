// I Language panic handler.
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

use std;

use crate::logging;


///////////////////
// PANIC HANDLER //
///////////////////

/// Generates a "crash" (panic) report file.
///
/// # Parameters
///
/// - `_location`: The panics location.
/// - `payload`: The downcasted panic payload.
/// - `backtrace`: The backtrace of the panic.
///
/// # Returns
///
/// The path of the report file.
///
/// # Examples
///
/// It is recommended to use [`setup_handler`] instead of the code from the example below.
///
/// ```rust
/// # use tools;
/// # use std;
/// std::panic::set_hook(Box::new(|panic_info: &std::panic::PanicInfo| {
///     tools::panic_handler::generate_report(
///         panic_info.location(),
///         panic_info.payload().downcast_ref::<String>().cloned(),
///         std::backtrace::Backtrace::force_capture()
///     );
/// }));
/// ```
///
/// Example report file:
///
/// ```txt,no_run
/// Crash report
/// ============
/// Reason: Problem with the code!
/// Location: src\main.rs:13
/// Log:
/// [2024-01-05T19:26:33Z TRACE icomp] Trace message
/// [2024-01-05T19:26:33Z ERROR icomp] Oh no, a bad thing happened!
/// Backtrace:
///    0: std::backtrace_rs::backtrace::dbghelp::trace
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library\std\src\..\..\backtrace\src\backtrace\dbghelp.rs:98
///    1: std::backtrace_rs::backtrace::trace_unsynchronized
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library\std\src\..\..\backtrace\src\backtrace\mod.rs:66
///    2: std::backtrace::Backtrace::create
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library\std\src\backtrace.rs:331
///    3: std::backtrace::Backtrace::force_capture
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library\std\src\backtrace.rs:313
///    4: I_Language::main::closure$0
///              at .\src\main.rs:6
///    5: alloc::boxed::impl$49::call
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library\alloc\src\boxed.rs:2021
///    6: std::panicking::rust_panic_with_hook
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library\std\src\panicking.rs:735
///    7: std::panicking::begin_panic_handler::closure$0
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library\std\src\panicking.rs:601
///    8: std::sys_common::backtrace::__rust_end_short_backtrace<std::panicking::begin_panic_handler::closure_env$0,never$>
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library\std\src\sys_common\backtrace.rs:170
///    9: std::panicking::begin_panic_handler
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library\std\src\panicking.rs:597
///   10: core::panicking::panic_fmt
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library\core\src\panicking.rs:72
///   11: I_Language::main
///              at .\src\main.rs:13
///   12: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962\library\core\src\ops\function.rs:250
///   13: std::sys_common::backtrace::__rust_begin_short_backtrace<void (*)(),tuple$<> >
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962\library\std\src\sys_common\backtrace.rs:154
///   14: std::sys_common::backtrace::__rust_begin_short_backtrace<void (*)(),tuple$<> >
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962\library\std\src\sys_common\backtrace.rs:154
///   15: std::rt::lang_start::closure$0<tuple$<> >
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962\library\std\src\rt.rs:166
///   16: std::rt::lang_start_internal::closure$2
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library\std\src\rt.rs:148
///   17: std::panicking::try::do_call
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library\std\src\panicking.rs:504
///   18: std::panicking::try
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library\std\src\panicking.rs:468
///   19: std::panic::catch_unwind
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library\std\src\panic.rs:142
///   20: std::rt::lang_start_internal
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library\std\src\rt.rs:148
///   21: std::rt::lang_start<tuple$<> >
///              at /rustc/79e9716c980570bfd1f666e3b16ac583f0168962\library\std\src\rt.rs:165
///   22: main
///   23: invoke_main
///              at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
///   24: __scrt_common_main_seh
///              at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
///   25: BaseThreadInitThunk
///   26: RtlUserThreadStart
/// ```
///
/// # See also
///
/// - [`setup_handler`]
pub fn generate_report(
    _location: Option<&std::panic::Location<'_>>,
    payload: Option<String>,
    backtrace: std::backtrace::Backtrace,
) -> Option<std::path::PathBuf> {
    let path: std::path::PathBuf = std::env::temp_dir().join("icomp_crash.txt");

    let header: String = "Crash report\n============\n".to_owned();
    let reason: String = if let Some(value) = payload {
        format!("Reason: {value}\n")
    } else {
        String::new()
    };
    let location: String = if let Some(value) = _location {
        format!("Location: {}:{}\n", value.file(), value.line())
    } else {
        String::new()
    };

    let log_buffer: String = format!("Log:\n{}\n", logging::buffer().join("\n"));
    let content: String = format!("Backtrace:\n{backtrace}");

    match std::fs::write(
        path.clone(),
        header
            + &reason
            + &location
            + if &log_buffer != "Log:\n\n" {
                &log_buffer
            } else {
                ""
            }
            + &content,
    ) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("Crash report generation failed.");
            eprintln!("File could not be created: {error}");
            return None;
        }
    };

    Some(path)
}

/// Sets the panic handler (hook) up.
///
/// # Examples
///
/// ```rust
/// # use tools;
/// tools::panic_handler::setup_handler();
/// ```
///
/// # See also
///
/// - [`generate_report`]
///
/// # Admonition
///
/// Inspired by [human-panic](https://crates.io/crates/human-panic).
pub fn setup_handler() {
    let panic_handler = |panic_info: &std::panic::PanicInfo| {
        eprintln!("Well, this is embarrassing...");
        eprintln!("The compiler had a problem and crashed. You can help us fix the error by voluntarily reporting the problem to us.");

        if let Some(path) = generate_report(
            panic_info.location(),
            panic_info.payload().downcast_ref::<String>().cloned(),
            std::backtrace::Backtrace::force_capture(),
        ) {
            eprintln!(
                "We have generated a crash report at \"{}\".",
                path.display()
            );
        }

        println!();
        println!("You can report the issue on:");
        println!(" - Our GitHub issue tracker: https://github.com/I-Language-Development/I-language-rust/issues/new/choose");
        println!(" - Our discord server: https://discord.gg/JVyyDukQqV");
    };
    std::panic::set_hook(Box::new(panic_handler));
}
