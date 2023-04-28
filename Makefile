all: run help
.PHONY: all

run:
	cargo run

help:
	echo "Usage: make [run]"
	echo ""
	echo "Options:"
	echo "    run: Compiles and executes the main.rs file"