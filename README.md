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

1. Work, including dependency updates (`just update`), lands on `sandbox`.
2. The operator bumps the version (patch, minor or major) in `Cargo.toml` and `Cargo.lock`
   with a tool of their choice, such as `cargo set-version --bump patch` or `gbump`, as
   the last commit of the work. The reviewer checks that the bump is there and right.
3. Merge `sandbox` into `develop` as a fast-forward so the history stays linear, push, and
   wait for `develop`'s **Test & Build** run to pass.
4. Run `just release`: it starts `.github/workflows/release.yml` on `develop` and follows
   the run, so a refusal by the workflow's guard shows in your terminal.

The workflow checks that the commit is green, that `Cargo.toml`'s version is above the
latest release and not tagged yet, and that `main` is an ancestor of it. It then builds
the Linux archives, RPMs and DEBs for x86_64 and aarch64 (musl, static) and the macOS
archives for x86_64 and Apple Silicon, packages the crate, attests the build
provenance, publishes the GitHub release (which creates the tag at that commit),
fast-forwards `main` to it, and publishes the crate through crates.io
[Trusted Publishing](https://crates.io/docs/trusted-publishing) (no stored token).

`Test & Build` already runs, on every push, what the release will do before anything is
published: the build, a run of the binary (`--version` must report the crate's version),
the RPM and DEB (checked for version and installed binary), and `cargo package` plus
`cargo publish --dry-run`. A problem there fails `develop` before you run `just release`,
so the release does not normally fail on content.

If a release job still fails, what to do depends on when:

- **Before the `GitHub release` job succeeds:** nothing is published and no tag exists.
  Fix the cause and use "Re-run failed jobs" (or fix the code, merge to `develop` and run
  `just release` again). A draft release left by a failed run is replaced.
- **After it (the tag exists, so `main` or crates.io failed):** the release and tag are
  immutable and cannot be moved or deleted. A transient failure is fixed by "Re-run failed
  jobs": the `main` job fast-forwards `main`, and the crates.io job skips a version it
  already has. If the fix needs a code change, bump to the next version and release that.

To verify a download:

```sh
sha256sum --check --ignore-missing SHA256SUMS
gh attestation verify <file> --repo zsmtp/zsmtp --signer-workflow zsmtp/zsmtp/.github/workflows/release.yml
```

Releasing needs `just` and `gh` (logged in).

`just hooks` enables the repository's git hooks (`.githooks`). Its `pre-push` hook refuses a
push to `develop` or `main` whose `Cargo.toml` version is not above the latest release tag
(`git push --no-verify` skips it). The release workflow checks the same thing in CI.

Third-party actions are pinned by full commit SHA (the tag is in the comment beside it);
Dependabot opens the update pull requests.
