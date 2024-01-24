# Set command aliases
alias b := build
alias c := clean
alias l := lint
alias r := run
alias t := test

# Remove directory command
remove_dir := if os_family() == "windows" { "rmdir /S /Q" } else { "rm -rf" }

# Support for windows
# /D: Don't execute auto run commands from registry
# /C: Execute command and terminate
set windows-shell := ["cmd.exe", "/D", "/C"]

# Rust backtrace
export RUST_BACKTRACE := "1"

# Compiles the rust source files
build *ARGUMENTS:
	@cargo build --release {{ARGUMENTS}}

# Removes temporary files
@clean:
	-{{remove_dir}} target
	-{{remove_dir}} {{join("Tools", "__pycache__")}}

# Lints the rust source files
lint *ARGUMENTS:
	@cargo check {{ARGUMENTS}}

# Compiles and executes the main.rs file
run *ARGUMENTS:
	@cargo run {{ARGUMENTS}}

# Runs the unittests
test *ARGUMENTS:
	@cargo test {{ARGUMENTS}}

# Update all submodules
update_submodules:
	@git submodule update --init --recursive --remote
	@echo Updated submodules.
