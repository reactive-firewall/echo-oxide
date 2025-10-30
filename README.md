# echo-oxide

A small, from-scratch Rust implementation of the Unix echo command.

echo-oxide aims to be a lightweight, correct, and well-tested reimplementation of the standard `echo` utility.
It mimics the common behaviors found in POSIX shells and GNU coreutils variants, while being implemented and distributed in Rust for clarity, safety, and portability.

* Language: Rust
* Goal: Provide a simple CLI that prints its arguments with configurable escaping and newline behavior.

## Features

* Print positional arguments separated by spaces.
* Default trailing newline after output.
* Common flags:
  * `-n` — do not print the trailing newline.
  * `-e` — enable interpretation of backslash escapes (e.g. `\n`, `\t`, `\\`).
  * `-E` — disable interpretation of backslash escapes (default behavior on many systems).
* Compatible behavior with typical echo semantics (see Notes / Compatibility).
* Minimal external dependencies.

## Badges

 > TODO

## Table of contents

Installation
Usage
Flags / Behavior
Examples
Testing
Development (building, linting, formatting)
Contributing
License
Acknowledgements
## Installation

* From source (requires Rust toolchain):

  1. Clone the repo: `git clone https://github.com/reactive-firewall/echo-oxide.git cd echo-oxide`
  2. Build in release mode: `cargo build --release`
  3. Optionally install the binary: `cargo install --path .`

* From crates.io (if published): `cargo install echo-oxide`

Usage Run the built binary directly or via cargo:

Using source-build installed binary: `echo [OPTIONS] [ARG...]`
Using crate.io installed binary: `echo-oxide [OPTIONS] [ARG...]`
With cargo run (development): `cargo run -- [OPTIONS] [ARG...]`

## Basic behavior:

* Print all provided arguments separated by a single space. By default, print a newline after the output unless suppressed with `-n`.

### Flags / Behavior

* `-n` Do not print the trailing newline.

* `-e` Enable interpretation of backslash escapes. Supported sequences typically include:

  * `\\` — backslash
  * `\a` — alert (BEL)
  * `\b` — backspace
  * `\c` — produce no further output
  * `\f` — form feed
  * `\n` — newline
  * `\r` — carriage return
  * `\t` — horizontal tab
  * `\v` — vertical tab
  * `\0NNN` — byte with octal value NNN (up to three digits)
  * `\xHH` — byte with hexadecimal value HH

* `-E` Explicitly disable interpretation of escape sequences (default for many shells).

## Notes / Compatibility

* `echo` semantics differ across shells and platforms (bash builtin vs. /bin/echo vs. GNU echo). `echo`-oxide tries to follow commonly expected behavior (drop-in for most BSD variants);
  including how `\c` is implemented, (e.g. output stops immediately and no trailing newline is printed).
* **Unicode handling**: strings are treated as UTF-8; escape sequences that produce bytes should be considered in terms of UTF-8 validity.

## Examples

**Basic**: `echo-oxide Hello world` _Output:_ `Hello world (with trailing newline)`

**No newline**: `echo-oxide -n Hello` _Output:_ `Hello (no newline)`

**Escapes**: `echo-oxide -e "Line1\nLine2"` _Output:_
  ```console
  Line1
  Line2
  ```

**Literal backslash**: `echo-oxide -e "A backslash: \\"` _Output:_ `A backslash: \`

**Stop output with `\c`**: `echo-oxide -e "Start\cShouldNotAppear"` _Output:_ `Start`

## Testing

* Run unit and integration tests: `cargo test`

## Development

* Build: `cargo build cargo build --release`

* Run: `cargo run -- [ARGS...]`

### Contributing

Contributions are welcome. Suggested workflow:

  1. Fork the repository.
  2. Create a branch for your feature/fix: `git checkout -b feature-my-stuff`
  3. Implement changes with tests.
  4. Run `cargo test`.
  5. Submit a pull request with a clear description and tests.

Please include tests for any new behavior and update the README where applicable.

### Reporting issues

Open a GitHub Issue in this repository with a clear title, description, steps to reproduce, and expected vs actual behavior. Tag any relevant platform / shell information to help reproduce differences.

## License

Your choice of:
Apache-2.0 or MIT-NFA

## Acknowledgements

* Inspired by the standard FreeBSD `echo` utility.
