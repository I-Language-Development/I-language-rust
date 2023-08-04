all: compile
.PHONY: all clean compile test help

clean:
	python3 Tools/clean.py

compile:
	cargo build

run:
	cargo run

test: src/main.rs
	cargo check

help:
	echo "Usage: make [command]"
	echo ""
	echo "Commands:"
	echo "    clean: Removes the 'target' directory and all files inside"
	echo "    compile: Compiles the rust source files"
	echo "    help: Shows this message"
	echo "    run: Compiles and executes the main.rs file"
	echo "    test: Runs test on the rust files"
