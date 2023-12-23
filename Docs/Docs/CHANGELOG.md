# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### [`v1.0.0-alpha4`](https://github.com/I-Language-Development/I-language-rust/releases/tag/v1.0.0-alpha4)

> 10/1/2023 - 11/2/2023

Currently in development.

#### BREAKING

- Nightly rust is now required. Use `just enable-lightly` to migrate

## Released

### [`v1.0.0-alpha3`](https://github.com/I-Language-Development/I-language-rust/releases/tag/v1.0.0-alpha3)

> 7/1/2023 - 8/2/2023

Added parser.

#### Added

- Added book
- Added pest parser ([gh-43](https://github.com/I-Language-Development/I-language-rust/pull/43))
- Added translations ([gh-43](https://github.com/I-Language-Development/I-language-rust/pull/43))
- Added dev docs ([gh-44](https://github.com/I-Language-Development/I-language-rust/pull/44))
- Added german installer translation ([gh-40](https://github.com/I-Language-Development/I-language-rust/issues/40))
- Added commit message linter

#### Changed

- Switched from gnu make to just
- Improved lexer ([gh-36](https://github.com/I-Language-Development/I-language-rust/pull/36))

#### Removed

- Removed lexer and replaced it with parser

#### Fixed

- Fixed issues regarding badges
- Fixed installer issues
- Fixed documentation issues

### [`v1.0.0-alpha2`](https://github.com/I-Language-Development/I-language-rust/releases/tag/v1.0.0-alpha2)

> 6/1/2023 - 7/2/2023

Added lexer.

#### Added

- Added parameters to windows installer tools
- Cloned [`f79f12b`](https://github.com/I-Language-Development/I-language-rust/commit/f79f12bff9c7661d212d5782df8ca7ffe72ba94f) from main
- Added ignore branches
- Added repository dustilock to megalinter ignore errors
- Added windows server installer ([gh-27](https://github.com/I-Language-Development/I-language-rust/issues/27))
- Added `.alz` files to the gitignore file
- Added linux installer ([gh-19](https://github.com/I-Language-Development/I-language-rust/issues/19))
- Added update section to the mergify configuration
- Added base branch configuration to the issue branch configuration
- Added changelog to the documentation ([gh-14](https://github.com/I-Language-Development/I-language-rust/issues/14))

#### Changed

- Modified file names
- Some changes
- Changed comments
- Changed header type
- Updated help about installation
- Changed comment type in batch files
- Centred image
- Updated documentation to fit new branch settings

#### Fixed

- Fixed jinja templates
- Fixed variable replacement in the documentation

### [`v1.0.0-alpha1`](https://github.com/I-Language-Development/I-language-rust/releases/tag/v1.0.0-alpha1)

> 5/15/2023 - 6/2/2023

Initial release. Does not contain lexer, parser or compiler.
