# zsmtp

[![Test & Build](https://github.com/zsmtp/zsmtp/actions/workflows/build.yml/badge.svg)](https://github.com/zsmtp/zsmtp/actions/workflows/build.yml)
[![codecov](https://codecov.io/github/zsmtp/zsmtp/graph/badge.svg?token=N8HAHTAFQ2)](https://codecov.io/github/zsmtp/zsmtp)
[![Crates.io](https://img.shields.io/crates/v/zsmtp.svg)](https://crates.io/crates/zsmtp)
[![License](https://img.shields.io/crates/l/zsmtp.svg)](https://github.com/zsmtp/zsmtp/main/LICENSE)


`zsmtp` is a zero-knowledge SMTP mail transfer agent prototype.

This repository now includes a Rust 2024 CLI skeleton derived from the modular architecture documented in `cron-when`, with placeholder commands and domain seams ready for protocol, thesis, and prototype work.

## Commands

```text
zsmtp serve
zsmtp submit --from alice@example.net --to bob@example.net --message-file mail.eml
zsmtp config show
zsmtp config validate
zsmtp doctor
```

## Configuration

`zsmtp` resolves configuration in this order:

1. `--config <PATH>`
2. `ZSMTP_CONFIG`
3. `./zsmtp.yaml`
4. `/etc/zsmtp/zsmtp.yaml`
5. built-in defaults

See [`examples/zsmtp.yaml`](examples/zsmtp.yaml) for the initial YAML shape.
