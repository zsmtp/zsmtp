# Contributing

We welcome contributions of all kinds.

## Getting Started

1. **Read the [Agent & Contributor Contract](AGENTS.md)**  
   This file contains the repository rules for code style, module boundaries, testing expectations, and operational safety.

2. **Pick an issue**  
   Check [TODO.md](TODO.md) if present, or review the open GitHub issues to find a task to work on.

3. **Run tests**  
   Use `just test` to run the standard local validation flow.

4. **Run linting**  
   This project uses strict Clippy rules. Run `just clippy` before submitting changes.

## Before Opening a Pull Request

- Keep changes focused and avoid mixing unrelated work.
- Run `cargo fmt --all -- --check` if you changed Rust code.
- Include a short summary of what changed and how you verified it.

## Notes

- Configuration examples live in `examples/`.
- CLI structure and architectural notes live in `CLI_ARCHITECTURE.md`.
- Research and prototype notes live in `docs/`.
