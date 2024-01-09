# Set command aliases
alias b := build
alias c := clean
alias f := format
alias i := install-binary
alias l := lint
alias r := run
alias t := test

alias fmt := format
alias install-dev-deps := install-dev-dependencies

# Remove directory command
remove_dir := if os_family() == "windows" { "rmdir /S /Q" } else { "rm -rf" }

# Support for windows
# /D: Don't execute auto run commands from registry
# /C: Execute command and terminate
set windows-shell := ["cmd.exe", "/D", "/C"]

# Rust backtrace
export RUST_BACKTRACE := "1"

# Audit the whole project (check for vulnerable dependencies)
# You may have to run `just install-dev-deps` before
audit *ARGUMENTS:
	@cargo audit --ignore RUSTSEC-2020-0138 {{ARGUMENTS}}

# Compiles the rust source files
build *ARGUMENTS:
	@cargo build --release --features cli {{ARGUMENTS}}

# Removes temporary files
@clean:
	cargo clean
	-{{remove_dir}} {{join("Tools", "__pycache__")}}
	git gc

# Format all source files
format *ARGUMENTS:
	@cargo +nightly fmt --all {{ARGUMENTS}}

# Install the `icomp` binary
install-binary:
	@cargo install --path .

# Installs the executable development dependencies
install-dev-dependencies:
	@cargo install cargo-audit

# Lints the rust source files
lint *ARGUMENTS:
	@cargo clippy --all-targets --features cli --workspace {{ARGUMENTS}}

# Compiles and executes the main.rs file
run *ARGUMENTS:
	@cargo run --features cli {{ARGUMENTS}}

# Runs the tests
test *ARGUMENTS:
	@cargo test --workspace {{ARGUMENTS}}

# Update all submodules
update-submodules:
	@git submodule update --init --recursive --remote
	@echo Updated submodules.
