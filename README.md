# Rust Log Analyzer

A lightweight command-line log analyzer written in Rust. It processes log files incrementally, summarizes common log levels, and identifies repeated error messages.

## Features

- Stream log files line by line
- Count INFO, WARN, ERROR, and unclassified lines
- Identify frequently repeated error messages
- Handle file and I/O errors
- Unit and integration tests
- No third-party dependencies

## Requirements

- Rust stable toolchain
- Cargo

## Installation

Clone the repository and enter the project directory:

```bash
git clone https://github.com/kraglin-labs/log_analyzer.git
cd log_analyzer
```

Build the project:

```bash
cargo build --release
```

## Usage

Analyze a log file:

```bash
cargo run -- sample.log
```

Run the tests:

```bash
cargo test
```

## Example

Given a log containing INFO, WARN, and ERROR entries, the analyzer prints a summary of the counts and the most frequent error messages.

## Design

The analysis engine uses Rust's standard library and buffered I/O to process input incrementally. The library and CLI are separated to make the core functionality independently testable.

## Current limitations

- Supports simple text logs with recognizable level tokens
- Does not yet parse structured JSON logs
- Does not yet support command-line filtering options

## Roadmap

- [ ] Level and keyword filtering
- [ ] Configurable number of top errors
- [ ] JSON output
- [ ] Benchmarks and performance documentation

## License

MIT
