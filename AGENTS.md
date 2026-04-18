# Repository Guidelines

## Project Structure & Module Organization

`zsmtp` is a Rust 2024 CLI project. Core code lives in `src/`:

- `src/bin/zsmtp.rs`: binary entrypoint
- `src/cli/`: CLI architecture (`commands`, `dispatch`, `actions`, `start`, `telemetry`)
- `src/config.rs`, `src/server.rs`, `src/message.rs`, `src/protocol.rs`, `src/crypto.rs`: domain stubs
- `src/lib.rs`: public module exports

Supporting files:

- `examples/zsmtp.yaml`: example configuration
- `docs/`: protocol, thesis, and prototype notes
- `.github/workflows/`: CI, build, coverage, and release automation
- `.justfile`: common local development tasks

## Build, Test, and Development Commands

Use Cargo directly or the `just` wrappers:

- `cargo build --release`: build optimized binary
- `cargo test --all-targets --all-features --locked`: run all tests
- `cargo clippy --all-targets --all-features -- -D warnings`: enforce lint policy
- `cargo fmt --all -- --check`: verify formatting
- `cargo package --locked`: verify publish/package metadata
- `just test`: runs `clippy`, `fmt`, and unit tests
- `just coverage`: generate coverage with `cargo llvm-cov`

## Coding Style & Naming Conventions

Follow standard Rust formatting with `rustfmt` and keep Clippy clean under the strict deny list in `Cargo.toml`. Use:

- `snake_case` for modules, files, functions, and variables
- `PascalCase` for types and enums
- small, focused modules with clear responsibility boundaries

Prefer explicit error handling with `anyhow`/`thiserror`; avoid `unwrap`, `expect`, and `panic`.

## Testing Guidelines

Unit tests live inline under `#[cfg(test)]` in the same file as the code they validate. Name tests descriptively, for example `dispatches_submit_action` or `load_reads_yaml_from_explicit_path`.

Before opening a PR, run:

- `cargo test --all-targets --all-features --locked`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo fmt --all -- --check`

## Commit & Pull Request Guidelines

The visible history is minimal (`Initial commit`), so keep commits short, imperative, and specific, for example `add smtp config validation` or `fix release workflow packaging`. Avoid mixing unrelated changes in one commit.

PRs should include:

- a brief problem/solution summary
- test or verification commands run
- linked issue or context when relevant
- notes on config, packaging, or workflow changes if they affect release behavior
