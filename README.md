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

## Releasing

A release ships the exact commit that **Test & Build** already passed on `develop`; the
release workflow bumps and commits nothing.

1. Work, including dependency updates (`just update`), lands on `sandbox`, then `develop`.
2. Bump the version as a normal commit on `develop` (`just bump`, or `just bump minor` /
   `major`) and wait for its **Test & Build** run to pass.
3. Run `just release` (it starts `.github/workflows/release.yml` on `develop`).

The workflow checks that the commit is green, that `Cargo.toml`'s version is not tagged
yet and that `main` is an ancestor of it. It then builds the Linux x86_64 musl archive,
RPM and DEB and the macOS x86_64 archive, packages the crate, attests the build
provenance, publishes the GitHub release (which creates the tag at that commit),
fast-forwards `main` to it, and publishes the crate through crates.io
[Trusted Publishing](https://crates.io/docs/trusted-publishing) (no stored token).

If a job fails, fix the cause and use "Re-run failed jobs" on the run: finished jobs are
not repeated, a draft release left by a failed run is replaced, and a version already on
crates.io is skipped. To verify a download:

```sh
sha256sum --check --ignore-missing SHA256SUMS
gh attestation verify <file> --repo zsmtp/zsmtp --signer-workflow zsmtp/zsmtp/.github/workflows/release.yml
```

Releasing needs `just`, `gh` (logged in) and `cargo-edit` for `just bump`.
