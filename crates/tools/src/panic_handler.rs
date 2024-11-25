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

use core::{fmt::Write as _, panic::Location};
use std::{
    backtrace::Backtrace,
    env::{consts::ARCH, temp_dir},
    fs::write,
    panic::{set_hook, PanicInfo},
    path::PathBuf,
    thread::sleep,
};

use crate::logging::BUFFER;

use sysinfo::{Cpu, System};

// Include build-time dependency list
include!(concat!(env!("OUT_DIR"), "/dependencies.rs"));


///////////////////
// PANIC HANDLER //
///////////////////

/// Generates a "crash" (panic) report file.
///
/// # Parameters
///
/// - `crash_location`: The panics location.
/// - `payload`: The downcasted panic payload.
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
///     );
/// }));
/// ```
///
/// Example report file:
///
/// ```txt,no_run
/// Crash report
/// ============
/// Location: src/main.rs:130
///
/// Version: 1.0.0-alpha.5
///
/// Crates (5):
///     - I-Language (9)
///         - clap@4.4.11
///         - clap-verbosity-flag@2.1.1
///         - localizer-rs@1.2.0
///         - log@0.4.20
///         - just@1.14.0
///     - compiler (0)
///     - grammar (0)
///     - lexer (4)
///         - annotate-snippets@0.10.0
///         - log@0.4.20
///         - thiserror@2.0.3
///     - tools (9)
///         - chrono@0.4.31
///         - clap-verbosity-flag@2.1.1
///         - current_locale@0.1.1
///         - env_logger@0.10.1
///         - localizer-rs@1.2.0
///         - log@0.4.20
///         - sysinfo@0.30.0
///         - cargo_metadata@0.18.1
///         - glob@0.3.1
///
/// OS: Linux 24.1.2 Manjaro Linux (6.6.54-2-MANJARO)
///
/// Hardware:
/// - CPU: 100.00%
///     - CPU 0: 11th Gen Intel(R) Core(TM) i3-1115G4 @ 3.00GHz (100.00%)
///     - CPU 1: 11th Gen Intel(R) Core(TM) i3-1115G4 @ 3.00GHz (100.00%)
///     - CPU 2: 11th Gen Intel(R) Core(TM) i3-1115G4 @ 3.00GHz (100.00%)
///     - CPU 3: 11th Gen Intel(R) Core(TM) i3-1115G4 @ 3.00GHz (100.00%)
/// - Memory: 6.53 / 7.51 GiB
/// - Architecture: x86_64
///
/// Log:
/// [2024-11-24T19:51:08Z TRACE icomp] Attempting to open file `examples/__dev.il`.
/// [2024-11-24T19:51:08Z TRACE icomp] Successfully opened file `examples/__dev.il`.
/// [2024-11-24T19:51:08Z TRACE lexer::lex] Lexing character " in line 1, column 1 took 0ms.
/// [2024-11-24T19:51:08Z TRACE lexer::lex] Lexing character " in line 1, column 3 took 0ms.
/// [2024-11-24T19:51:08Z TRACE lexer::lex] Lexing character " in line 1, column 8 took 0ms.
/// [2024-11-24T19:51:08Z DEBUG icomp] Lexing `examples/__dev.il` took 0ms.
///
/// Backtrace:
///    0: tools::panic_handler::generate_report
///              at ./crates/tools/src/panic_handler.rs:232:57
///    1: tools::panic_handler::setup_handler::{{closure}}
///              at ./crates/tools/src/panic_handler.rs:284:29
///    2: <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call
///              at /rustc/a70b2ae57713ed0e7411c059d582ab382fc4166a/library/alloc/src/boxed.rs:2077:9
///    3: std::panicking::rust_panic_with_hook
///              at /rustc/a70b2ae57713ed0e7411c059d582ab382fc4166a/library/std/src/panicking.rs:799:13
///    4: std::panicking::begin_panic_handler::{{closure}}
///              at /rustc/a70b2ae57713ed0e7411c059d582ab382fc4166a/library/std/src/panicking.rs:656:13
///    5: std::sys_common::backtrace::__rust_end_short_backtrace
///              at /rustc/a70b2ae57713ed0e7411c059d582ab382fc4166a/library/std/src/sys_common/backtrace.rs:171:18
///    6: rust_begin_unwind
///              at /rustc/a70b2ae57713ed0e7411c059d582ab382fc4166a/library/std/src/panicking.rs:652:5
///    7: core::panicking::panic_fmt
///              at /rustc/a70b2ae57713ed0e7411c059d582ab382fc4166a/library/core/src/panicking.rs:72:14
///    8: icomp::main
///              at ./src/main.rs:130:13
///    9: core::ops::function::FnOnce::call_once
///              at /rustc/a70b2ae57713ed0e7411c059d582ab382fc4166a/library/core/src/ops/function.rs:250:5
///   10: std::sys_common::backtrace::__rust_begin_short_backtrace
///              at /rustc/a70b2ae57713ed0e7411c059d582ab382fc4166a/library/std/src/sys_common/backtrace.rs:155:18
///   11: std::rt::lang_start::{{closure}}
///              at /rustc/a70b2ae57713ed0e7411c059d582ab382fc4166a/library/std/src/rt.rs:159:18
///   12: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once
///              at /rustc/a70b2ae57713ed0e7411c059d582ab382fc4166a/library/core/src/ops/function.rs:284:13
///   13: std::panicking::try::do_call
///              at /rustc/a70b2ae57713ed0e7411c059d582ab382fc4166a/library/std/src/panicking.rs:559:40
///   14: std::panicking::try
///              at /rustc/a70b2ae57713ed0e7411c059d582ab382fc4166a/library/std/src/panicking.rs:523:19
///   15: std::panic::catch_unwind
///              at /rustc/a70b2ae57713ed0e7411c059d582ab382fc4166a/library/std/src/panic.rs:149:14
///   16: std::rt::lang_start_internal::{{closure}}
///              at /rustc/a70b2ae57713ed0e7411c059d582ab382fc4166a/library/std/src/rt.rs:141:48
///   17: std::panicking::try::do_call
///              at /rustc/a70b2ae57713ed0e7411c059d582ab382fc4166a/library/std/src/panicking.rs:559:40
///   18: std::panicking::try
///              at /rustc/a70b2ae57713ed0e7411c059d582ab382fc4166a/library/std/src/panicking.rs:523:19
///   19: std::panic::catch_unwind
///              at /rustc/a70b2ae57713ed0e7411c059d582ab382fc4166a/library/std/src/panic.rs:149:14
///   20: std::rt::lang_start_internal
///              at /rustc/a70b2ae57713ed0e7411c059d582ab382fc4166a/library/std/src/rt.rs:141:20
///   21: std::rt::lang_start
///              at /rustc/a70b2ae57713ed0e7411c059d582ab382fc4166a/library/std/src/rt.rs:158:17
///   22: main
///   23: <unknown>
///   24: __libc_start_main
///   25: _start
/// ```
///
/// # See also
///
/// - [`setup_handler`]
#[inline(always)] // Suggesting inlining due to rare calls to the function
#[allow(
    clippy::as_conversions,
    clippy::cast_precision_loss,
    clippy::float_arithmetic
)] // Allowing for the byte --> GiB conversion
pub fn generate_report(
    crash_location: Option<&Location<'_>>,
    payload: Option<String>,
) -> Option<PathBuf> {
    let path: PathBuf = temp_dir().join("icomp_crash.txt");

    let header: String = "Crash report\n============\n".to_owned();
    let reason: String = if let Some(value) = payload {
        format!("Reason: {value}\n")
    } else {
        String::new()
    };
    let location: String = if let Some(value) = crash_location {
        format!("Location: {}:{}\n", value.file(), value.line())
    } else {
        String::new()
    };

    let version: String = format!("\nVersion: {}\n", env!("CARGO_PKG_VERSION"));

    let mut dependencies: String = if DEPENDENCIES.is_empty() {
        String::new()
    } else {
        format!("\nCrates ({}):\n", DEPENDENCIES.len())
    };
    for &(crate_name, dependency_list) in DEPENDENCIES {
        let _ = writeln!(
            dependencies,
            "    - {crate_name} ({})",
            dependency_list.len()
        );

        for &(name, dependency_version) in dependency_list {
            if !DEPENDENCIES
                .iter()
                .any(|&(dependency_name, _)| name == dependency_name)
            {
                let _ = writeln!(
                    dependencies,
                    "        - {name}@{}",
                    dependency_version.trim_start_matches('^')
                );
            }
        }
    }

    let log_buffer: String = format!(
        "\nLog:\n{}\n",
        match BUFFER.lock() {
            Ok(value) => value.join("\n"),
            Err(_) => String::new(),
        }
    );

    let mut system: System = System::new_all();
    let os_info: String = format!(
        "\nOS: {} ({})\n",
        System::long_os_version().unwrap_or("unknown".to_owned()),
        System::kernel_version().unwrap_or("unknown".to_owned())
    );

    // Wait for the CPU to update (see https://docs.rs/sysinfo/latest/sysinfo/constant.MINIMUM_CPU_UPDATE_INTERVAL.html)
    sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    system.refresh_cpu();
    let global_cpu: &Cpu = system.global_cpu_info();

    let mut cpu_info: String = format!("- CPU: {:.2}%\n", global_cpu.cpu_usage());
    for (index, cpu) in system.cpus().iter().enumerate() {
        let _ = writeln!(
            cpu_info,
            "    - CPU {}: {} ({:.2}%)",
            index,
            cpu.brand(),
            cpu.cpu_usage()
        );
    }
    let memory_info: String = format!(
        "- Memory: {:.2} / {:.2} GiB\n",
        system.used_memory() as f64 / (1024_f64.powi(3)),
        system.total_memory() as f64 / (1024_f64.powi(3))
    );
    let architecture: String = format!("- Architecture: {ARCH}\n");

    let backtrace: String = format!("\nBacktrace:\n{}", Backtrace::force_capture());

    let mut content: String = String::new();
    content.push_str(&header);
    content.push_str(&reason);
    content.push_str(&location);
    content.push_str(&version);
    content.push_str(&dependencies);
    content.push_str(&os_info);
    content.push_str("\nHardware:\n");
    content.push_str(&cpu_info);
    content.push_str(&memory_info);
    content.push_str(&architecture);
    content.push_str(if &log_buffer == "Log:\n\n" {
        ""
    } else {
        &log_buffer
    });
    content.push_str(&backtrace);

    match write(path.clone(), content) {
        Ok(()) => Some(path),
        Err(error) => {
            eprintln!("Crash report generation failed.");
            eprintln!("File could not be created: {error}");
            None
        }
    }
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
#[inline(always)] // Suggesting inlining due to rare calls to the function
pub fn setup_handler() {
    let panic_handler = |panic_info: &PanicInfo| {
        eprintln!("Well, this is embarrassing...");
        eprintln!("The compiler had a problem and crashed. You can help us fix the error by voluntarily reporting the problem to us.");

        if let Some(path) = generate_report(
            panic_info.location(),
            panic_info.payload().downcast_ref::<String>().cloned(),
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
    set_hook(Box::new(panic_handler));
}
