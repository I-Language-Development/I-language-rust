all: run help
.PHONY: all clean test

clean:
	python Tools/clean.py

run:
	cargo run

test:
	cargo check

help:
	echo "Usage: make [run]"
	echo ""
	echo "Options:"
	echo "    clean: Removes the 'target' directory and all files inside"
	echo "    help: Shows this message"
	echo "    run: Compiles and executes the main.rs file"
	echo "    test: Runs test on the rust files"
