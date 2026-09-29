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

A release is one command, `just deploy`, and follows one rule: **the commit that is
tagged and put on `main` is exactly the commit CI tested, and everything published is
exactly what CI built.** The version bump is staged on the scratch `release` branch,
Test & Build and a candidate run of the Release workflow (`.github/workflows/release.yml`)
test and package that exact commit, and only then do `develop`, `main` and the signed
tag move together in one atomic push. The tag's run builds nothing: it publishes the
candidate's files and crate. A failure before the tag leaves nothing to clean up, and a
release never needs a tag deleted or moved. The flow comes from
[cron-when](https://github.com/nbari/cron-when), whose
[RELEASING.md](https://github.com/nbari/cron-when/blob/main/RELEASING.md) explains it
in full: the diagrams, every failure and what to do, the guarantees and limits, and how
to verify a download.

Work, including dependency updates (`just update`), lands on `sandbox`. When its
**Test & Build** run is green, merge it into `develop` and run `just deploy` from a clean
`develop`; you can keep working on `sandbox` meanwhile.

| Command | What it does |
|---|---|
| `just deploy` | Release a patch version (`deploy-minor`, `deploy-major` for the others); when `develop` already carries an untagged version, that version is released as is |
| `just deploy-current` | Release `develop`'s untagged version as is, explicitly |
| `just release-status` | Show `develop`, `main`, `sandbox`, the staged candidate and its runs |
| `just release-preflight` | Run only the checks; changes nothing that lasts |
| `just release-dry-run` | Build and package the current branch exactly like a candidate, releasing nothing: no bump, no tag |
| `just release-republish X.Y.Z` | Recovery: publish an existing tag again with `main`'s workflow |
| `just protect-branches` | Apply the branch protection and the rule that release tags are never moved or deleted |

The candidate run builds the Linux x86_64 musl archive, RPM and DEB and the macOS x86_64 archive, packages and verifies the crate, keeps everything
with a manifest of SHA-256 sums, and attests the build provenance of every file. The tag
run publishes those files to the GitHub release with notes made from the commit
subjects since the previous release, marks it Latest only while it is the highest
release, and uploads the crate with crates.io
[Trusted Publishing](https://crates.io/docs/trusted-publishing) (no stored token),
only when the repackaged crate has the tested checksum. To check a download:

```sh
sha256sum --check --ignore-missing SHA256SUMS
gh attestation verify <file> --repo zsmtp/zsmtp --signer-workflow zsmtp/zsmtp/.github/workflows/release.yml
```

If anything fails before promotion, nothing moved: re-run the failed jobs in GitHub, or
fix it on `sandbox` and merge, then run `just deploy` again; it resumes the same
candidate. A failed publish step is finished by "Re-run failed jobs" on the tag run,
within GitHub's 30-day window, or later with `just release-republish X.Y.Z` while the
candidate's artifacts are kept (90 days). Releasing needs `just`, `jq`, `gh` (logged
in), git 2.31 or later, `cargo-edit`, and a signing key that GitHub knows as a signing
key, since commits and tags must be signed.
