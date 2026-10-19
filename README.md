# Rust Log Analyzer

A lightweight command-line log analyzer written in Rust. It processes log files
incrementally, summarizes log levels, and identifies repeated error messages.
Supports both plain-text logs (e.g. `[INFO] ...`, `ERROR: ...`) and structured
JSON/JSONL logs.

## Features

- Stream log files line by line with buffered I/O
- Parse plain-text logs with level prefixes (`[ERROR]`, `INFO:`, `WARN ...`)
- Parse JSON/JSONL logs, accepting `level`/`severity` and `message`/`msg` fields
- Count TRACE, DEBUG, INFO, WARN, ERROR, FATAL, malformed JSON, and other lines
- Identify the most frequently repeated error messages
- Library/CLI split: the analysis core is a testable library, `main.rs` is a thin CLI
- Unit + integration tests, run on every push via GitHub Actions CI

## Dependencies

- `serde_json` for JSON log parsing

## Usage

```bash
# build
cargo build --release

# analyze a plain-text log
cargo run -- sample.log

# analyze a JSONL log
cargo run -- sample.jsonl

# run the tests
cargo test
