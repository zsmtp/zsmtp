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

Releases use the [release](https://github.com/nbari/release) framework, pinned by full
SHA in `.release-flow.json` and both workflow callers. A candidate is built once from an
exact commit, verified against its manifest, retained, and promoted as that same commit;
publishing reuses the retained bytes and never rebuilds. The project-specific part is
`tools/adapter.sh`: the framework's Cargo adapter plus the `.crate` artifact.

Work, including dependency updates (`just update`), lands on `sandbox`. When its
**Test & Build** run is green, merge it into `develop` and prepare a candidate from a
clean `develop`:

```sh
just deploy                       # patch; deploy-minor / deploy-major for the others
just release-candidates
just release-status X.Y.Z --json
just release-promote release/X.Y.Z-<base12> <candidate-sha> --dry-run
just release-promote release/X.Y.Z-<base12> <candidate-sha>
```

`just deploy-test` (from `sandbox`) builds a `release/test/...` candidate that cannot be
promoted. Recovery: `just release-retry X.Y.Z <channel>`; cleanup: `just release-prune --dry-run`.

The candidate workflow (`.github/workflows/candidate.yml`) runs fmt, clippy and tests,
then packages the Linux x86_64 musl archive, RPM and DEB, the macOS x86_64 archive and
the crate. The signed tag triggers `.github/workflows/release.yml`, which publishes the
GitHub release and, as an optional channel, crates.io. Channels are configured under
`publishers` in `.release-flow.json`.

Releasing needs `just`, `jq`, `gh` (logged in), git 2.31 or later and a signing key that
GitHub knows as a signing key. One-time repository setup (rulesets, the `candidate-storage`
and `publish-*` environments, GitHub storage bootstrap) is described in the framework's
[adoption guide](https://github.com/nbari/release/blob/main/docs/ADOPTION.md).
