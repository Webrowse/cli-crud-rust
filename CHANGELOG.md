# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.1.0] - 2025-07-05

### Added
- Initial release of `bt`: a minimal CLI task manager written in pure Rust.
- Supports four commands:
  - `add <task>` — Adds a new task as `[ ] <task>`.
  - `list` — Lists all current tasks with line numbers.
  - `complete <n>` — Toggles completion state of task number `n`.
  - `delete <n>` — Deletes task number `n` from list.
- Stores data in a local `Tasks.txt` file.
- No dependencies; does not use `serde`, `clap`, or external crates.
- Includes basic error handling and inline unit tests.

