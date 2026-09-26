# `zsmtp` CLI Architecture

This document describes the modular CLI architecture for the `zsmtp` SMTP/MTA prototype.

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
└── telemetry.rs       # Local logging and optional OTLP tracing
```

## Data Flow

```text
bin/zsmtp.rs
    ↓
cli::start()
    ↓
1. commands::new().get_matches()
2. Convert the verbose count to a tracing level
3. telemetry::init(level)
4. dispatch::handler(&matches)
5. binary matches Action and calls action executor
6. executor loads config/domain stubs and returns structured output
```

## Design Notes

- `commands` owns argument shape only.
- `dispatch` is the single source of truth for converting CLI arguments into typed actions.
- `actions` executes placeholder behavior and is the only CLI layer allowed to call domain modules.
- `config`, `server`, `message`, `protocol`, and `crypto` are intentionally broad stubs so later protocol and thesis work can grow without reshaping the crate.
- `telemetry` owns logging setup and tracer shutdown. Local `tracing-subscriber` logging is available in every build.

## Telemetry Feature

The default-off Cargo feature named `telemetry` adds OpenTelemetry and OTLP
dependencies for distributed tracing. Those dependencies are optional and
grouped under the feature. The baseline build continues to initialize local
logging and does not include an OTLP exporter.

| Build | Startup configuration | Behavior |
| --- | --- | --- |
| `cargo build` | Any | Local logging only |
| `cargo build --features telemetry` | No `OTEL_EXPORTER_OTLP_ENDPOINT` | Local logging only |
| `cargo build --features telemetry` | `OTEL_EXPORTER_OTLP_ENDPOINT` set | Local logging and OTLP trace export |

The Cargo feature determines what is compiled into the binary. The endpoint
determines whether an enabled binary creates an exporter at startup. Changing
the feature requires a rebuild; changing the endpoint requires a process
restart. When enabled, the exporter uses OTLP over gRPC with gzip and TLS
support. `OTEL_EXPORTER_OTLP_HEADERS` supplies optional request headers.
Telemetry builds use Tokio's multi-thread runtime so trace export can finish
while the CLI flushes its spans.
`telemetry::shutdown_tracer()` flushes an initialized provider before the
process exits and otherwise returns without work.

The feature is additive. The test workflow checks the all-feature build, while
the release workflow builds without `--features telemetry`, so published
binaries include local logging only.
