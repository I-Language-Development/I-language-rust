# Set command aliases
alias b := build
alias c := clean
alias f := format
alias i := install-binary
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
	cargo clean
	-{{remove_dir}} {{join("Tools", "__pycache__")}}
	git gc

# Enable nightly version of rust
enable-nightly:
	@rustup override set nightly

# Format all source files
format *ARGUMENTS:
	@cargo fmt --all {{ARGUMENTS}}

# Install the `ilang` binary
install-binary:
	@cargo install --path .

# Lints the rust source files
lint *ARGUMENTS:
	@cargo check {{ARGUMENTS}}

# Compiles and executes the main.rs file
run *ARGUMENTS:
	@cargo run {{ARGUMENTS}}

# Runs the tests
test *ARGUMENTS:
	@cargo test -p I-Language -p I-Language-lexer -p I-Language-tools {{ARGUMENTS}}

# Update all submodules
update-submodules:
	@git submodule update --init --recursive --remote
	@echo Updated submodules.
