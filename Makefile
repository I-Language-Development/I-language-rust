all: run help
.PHONY: all test clean

run:
	cargo run

test:
	cargo check

clean:
	python Tools/clean.py

help:
	echo "Usage: make [run]"
	echo ""
	echo "Options:"
	echo "    run: Compiles and executes the main.rs file"