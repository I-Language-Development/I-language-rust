# Source code documentation

!!! note

    The source can be found [here](https://github.com/I-Language-Development/I-language-rust).

These files contain the source code documentation, meaning documentation about public and private functions or classes, variables and more.

## List

| Name                                                                                                        | Type                       | Description                                                                            |
| ----------------------------------------------------------------------------------------------------------- | -------------------------- | -------------------------------------------------------------------------------------- |
| [`lib.rs`](./lib.rs.md)                                                                                     | Rust source code           | Contains "links" to dependencies of `main.rs`                                   |
| [`main.rs`](./main.rs.md)                                                                                   | Rust source code           | Main source code file, combines lexer, parser, compiler and VM and loads CLI arguments |
| [`Compiler/compiler.rs`](./Compiler/compiler.rs.md)                                                         | Rust source code           | Compiles the AST given by the parser                                                   |
| [`Compiler/mod.rs`](./Compiler/mod.rs.md)                                                                   | Rust source code           | File allowing `compiler.rs` to be imported everywhere                                  |
| [`Grammar/fileextensions`](./Grammar/fileextensions.md)                                                     | File extension list        | File extensions allowed for I Language files                                           |
| [`Grammar/grammar.pest`](./Grammar/grammar.pest.md)                                                         | Pest grammar specification | Grammar for the language                                                               |
| [`Grammar/grammar.rs`](./Grammar/grammar.rs.md)                                                             | Rust source code           | Reads the `grammar.pest` file and processes it                                         |
| [`Grammar/mod.rs`](./Grammar/mod.rs.md)                                                                     | Rust source code           | File allowing `grammar.rs` to be imported everywhere                                   |
| [`Installer/Linux/installer.bash`](./Installer/Linux/installer.bash.md)                                     | Bash script                | Linux command line installer                                                           |
| [`Installer/Windows/add_desktop_icon.bat`](./Installer/Windows/add_desktop_icon.bat.)                       | Batch script               | Adds a desktop shortcut                                                                |
| [`Installer/Windows/add_to_path.bat`](./Installer/Windows/add_to_path.bat.md)                               | Batch script               | Adds a path to the variable `PATH`.                                                    |
| [`Installer/Windows/associate_file_extension.bat`](./Installer/Windows/associate_file_extension.bat.md)     | Batch script               | Associates `.il` with the I language compiler                                          |
| [`Installer/Windows/setup.iss`](./Installer/Windows/setup.iss.md)                                           | Inno setup file            | Configures inno setup to make a windows GUI installer                                  |
| [`Installer/Windows/unassociate_file_extension.bat`](./Installer/Windows/unassociate_file_extension.bat.md) | Batch script               | Unassociates `.il` with the I language compiler                                        |
| [`Installer/Windows/Server/installer.bat`](./Installer/Windows/Server/installer.bat.md)                     | Batch script               | Windows command line installer                                                         |
| [`Lexer/lexer.rs`](./Lexer/lexer.rs.md)                                                                     | Rust source code           | Lexes the code given                                                                   |
| [`Lexer/mod.rs`](./Lexer/mod.rs.md)                                                                         | Rust source code           | File allowing `lexer.rs` to be imported everywhere                                     |
| [`Parser/mod.rs`](./Parser/mod.rs.md)                                                                       | Rust source code           | File allowing `parser.rs` to be imported everywhere                                    |
| [`Parser/parser.rs`](./Parser/parser.rs.md)                                                                 | Rust source code           | Parses the lexer output into an AST                                                    |
