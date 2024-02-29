# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [`v1.0.0-alpha.4`](https://github.com/I-Language-Development/I-language-rust/releases/tag/v1.0.0-alpha4) - 02/29/2024

### Added

- lexer: Added custom error type (([#103](https://github.com/I-Language-Development/I-language-rust/issues/103))) by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`503a789`](https://github.com/I-Language-Development/I-language-rust/commit/503a789).
- ci: Added `just install-dev-dependencies` to workflows by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`91ae84f`](https://github.com/I-Language-Development/I-language-rust/commit/91ae84f).
- cli: Switched from `.try_exists()` to `.exists()` by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`d2a2bca`](https://github.com/I-Language-Development/I-language-rust/commit/d2a2bca).
- lexer: Added `Display` trait to all token types by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`20fd611`](https://github.com/I-Language-Development/I-language-rust/commit/20fd611).
- lexer: Added `Default` trait to `Location` by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`466d4ee`](https://github.com/I-Language-Development/I-language-rust/commit/466d4ee).
- lexer: Added as keyword by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`23f8211`](https://github.com/I-Language-Development/I-language-rust/commit/23f8211).
- errors: Added errors and more logging to the lexer and compiler cli by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`d67bfe8`](https://github.com/I-Language-Development/I-language-rust/commit/d67bfe8).

### Changed

- Removed add-desktop-icon script due to it not being needed by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`3aecb8d`](https://github.com/I-Language-Development/I-language-rust/commit/3aecb8d).
- Removed outdated modules, will be added back tho by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`668ffdf`](https://github.com/I-Language-Development/I-language-rust/commit/668ffdf).
- Finishing renaming files by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`ffb1640`](https://github.com/I-Language-Development/I-language-rust/commit/ffb1640).
- Prepared files to switch to lowercase globally by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`72de5d1`](https://github.com/I-Language-Development/I-language-rust/commit/72de5d1).
- Renamed `Boolean` variant on `Constant` enum for consitency with the other variants by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`c6a4b1e`](https://github.com/I-Language-Development/I-language-rust/commit/c6a4b1e).

### Removed

- Removed delete keyword by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`b202089`](https://github.com/I-Language-Development/I-language-rust/commit/b202089).

### Fixed

- lexer: Fixed constant quote_type being used by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`d1b0320`](https://github.com/I-Language-Development/I-language-rust/commit/d1b0320).
- ci: Fixed dev dependencies not installing by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`e9a0f0b`](https://github.com/I-Language-Development/I-language-rust/commit/e9a0f0b).
- Fixed megalinter still being setup with the uppercase file names by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`be6b4d1`](https://github.com/I-Language-Development/I-language-rust/commit/be6b4d1).
- New paths by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`c51ec4a`](https://github.com/I-Language-Development/I-language-rust/commit/c51ec4a).
- cargo: `cargo-audit` is an optional dependency by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`ce6b7c9`](https://github.com/I-Language-Development/I-language-rust/commit/ce6b7c9).
- Fix github workflows using old paths by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`84c4f97`](https://github.com/I-Language-Development/I-language-rust/commit/84c4f97).
- FIx justfile using old paths by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`9057d92`](https://github.com/I-Language-Development/I-language-rust/commit/9057d92).
- Added back the submodule by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`a865bec`](https://github.com/I-Language-Development/I-language-rust/commit/a865bec).
- ci: Fixed repository link containing an excess slash by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`686f22b`](https://github.com/I-Language-Development/I-language-rust/commit/686f22b).
- lexer: Fixed lexer not allowing `"\\"` by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`45df4e3`](https://github.com/I-Language-Development/I-language-rust/commit/45df4e3).
- git: Add more files to .gitignore by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`985119f`](https://github.com/I-Language-Development/I-language-rust/commit/985119f).

### Documentation

- Updated `git-cliff` to v2.0.2 and used the new features by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`0fdb993`](https://github.com/I-Language-Development/I-language-rust/commit/0fdb993).
- Fixed using the wrong name by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`24f63a9`](https://github.com/I-Language-Development/I-language-rust/commit/24f63a9).
- Added changelog generator by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`3a049f9`](https://github.com/I-Language-Development/I-language-rust/commit/3a049f9).
- tools: Added module documentation by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`bf7e84b`](https://github.com/I-Language-Development/I-language-rust/commit/bf7e84b).
- Updated submodules by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`487ef4c`](https://github.com/I-Language-Development/I-language-rust/commit/487ef4c).

### Styling

- Fixed editorconfig issues by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`83d68da`](https://github.com/I-Language-Development/I-language-rust/commit/83d68da).

### Other

- ci: Removed release workflow by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`686ed68`](https://github.com/I-Language-Development/I-language-rust/commit/686ed68).
- Removed accidentally committed directory by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`f25243d`](https://github.com/I-Language-Development/I-language-rust/commit/f25243d).
- ci: Changed commit prefix for removals by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`98fbdbf`](https://github.com/I-Language-Development/I-language-rust/commit/98fbdbf).
- Made dependabot use conventional commits by [@ElBe-Plaq](https://github.com/ElBe-Plaq)<!-- SEPARATOR --> in [`48ca651`](https://github.com/I-Language-Development/I-language-rust/commit/48ca651).

For the old changelog from `v1.0.0-alpha1` to `v1.0.0-alpha3`, see the old [`CHANGELOG.md`](https://github.com/I-Language-Development/I-language-rust/blob/76abd0edb5e1636a90421e424b35c0c6645921eb/Docs/Docs/CHANGELOG.md).
