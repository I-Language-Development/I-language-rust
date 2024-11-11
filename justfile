# Set command aliases
alias a := audit
alias b := build
alias c := clean
alias d := doc
alias f := format
alias i := install-binary
alias l := lint
alias r := run
alias t := test

alias cl := update-changelog
alias id := install-doc-requirements
alias sd := serve-docs
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

# for line 27: See rustsec/rustsec#696
# Audit the whole project (check for vulnerable dependencies)
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

doc *ARGUMENTS:
	@cargo doc --no-deps --all-features --workspace {{ARGUMENTS}}

# Format all source files
format *ARGUMENTS:
	@cargo +nightly fmt --all {{ARGUMENTS}}

# Install the `icomp` binary
install-binary:
	@cargo install --path .

# Installs the executable development dependencies (this is gonna take a while)
install-dev-dependencies:
	@cargo install git-cliff
	@cargo install cargo-audit

# Install documentation dependencies
install-doc-requirements:
	@pip install -r docs\.requirements.txt

# Install pre-commit
install-pre-commit:
	@pip install -r dev-requirements.txt
	@pre-commit install --install-hooks

# See rust-lang/cargo/12918 for why `-A clippy::std-instead-of-core` is needed
# Lints the rust source files
lint *ARGUMENTS:
	@cargo clippy --all-targets --all-features --workspace -- -A clippy::std-instead-of-core {{ARGUMENTS}}

# Compiles and executes the main.rs file
run *ARGUMENTS:
	@cargo run --features cli {{ARGUMENTS}}

# Serve documentation locally
serve-docs *ARGUMENTS:
	@mkdocs serve -f docs\.mkdocs.yml {{ARGUMENTS}}

# Runs the tests
test *ARGUMENTS:
	@cargo test --workspace {{ARGUMENTS}}

# Updated the changelog using git-cliff
update-changelog *ARGUMENTS:
	@git cliff -c .github\cliff.toml -o docs\docs\CHANGELOG.md {{ARGUMENTS}}
	@echo Updated changelog.

# Update all submodules
update-submodules:
	@git submodule update --init --recursive --remote
	@echo Updated submodules.
