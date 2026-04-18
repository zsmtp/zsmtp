# `zsmtp` CLI Architecture

This project uses the modular CLI architecture documented in `cron-when`, adapted here for an SMTP/MTA prototype with a wider domain surface.

## Directory Structure

```text
src/cli/
├── actions/           # Action definitions and execution
│   ├── mod.rs         # Action enum and shared output types
│   ├── serve.rs       # Placeholder relay/server execution
│   ├── submit.rs      # Placeholder message submission execution
│   ├── config.rs      # Config show/validate execution
│   └── doctor.rs      # Environment/config diagnostics
├── commands/          # CLI definition with clap
│   └── mod.rs
├── dispatch/          # ArgMatches -> Action conversion
│   └── mod.rs
├── mod.rs             # Module exports
├── start.rs           # CLI startup orchestration
└── telemetry.rs       # Tracing/log initialization
```

## Data Flow

```text
bin/zsmtp.rs
    ↓
cli::start()
    ↓
1. commands::new().get_matches()
2. telemetry::init(level)
3. dispatch::handler(&matches)
4. binary matches Action and calls action executor
5. executor loads config/domain stubs and returns structured output
```

## Design Notes

- `commands` owns argument shape only.
- `dispatch` is the single source of truth for converting CLI arguments into typed actions.
- `actions` executes placeholder behavior and is the only CLI layer allowed to call domain modules.
- `config`, `server`, `message`, `protocol`, and `crypto` are intentionally broad stubs so later protocol and thesis work can grow without reshaping the crate.
- `telemetry` stays lightweight for now, but keeps the same startup seam where richer OpenTelemetry support can be added later.
