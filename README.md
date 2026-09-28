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

A release follows one rule: **the commit that is tagged and put on `main` is exactly the
commit CI tested, and the published files are exactly the files CI built.** `just deploy`
promotes a commit only after every test, build and package passed on it, so a release
never needs a tag deleted or moved. Branch protection lets onto `main` only commits whose
**CI OK** check passed, and the Release workflow publishes a tag only with the successful
candidate run the signed tag names. The flow lives in `scripts/release`,
`.github/workflows/build.yml` (Test & Build) and `.github/workflows/release.yml`
(Release); it is the flow documented in full in the "Releasing" section of
[cron-when](https://github.com/nbari/cron-when#releasing), which is its template.

Work, including dependency updates (`just update`), lands on `sandbox`. When its
**Test & Build** run is green, merge it into `develop` and run `just deploy` from a clean
`develop`.

| Command | What it does |
|---|---|
| `just deploy` | Release a patch bump (`deploy-minor`, `deploy-major` for the others); when `develop`'s current version has no tag yet, it releases that version as is instead |
| `just deploy-current` | Release `develop`'s untagged version as is, explicitly |
| `just release-status` | Show `develop`, `main`, the staged candidate, its runs and the last tag's publish run |
| `just release-preflight` | Run only the checks; changes nothing apart from fetching |
| `just release-republish X.Y.Z` | Recovery: publish an existing tag again with `main`'s workflow |
| `just protect-branches` | Apply the branch protection the flow relies on |
| `just t-deploy` | Push a `t-*` test tag: tests, builds and packages, publishes nothing |

`just deploy` checks that `develop` is clean and equal to origin, then bumps the version
(`Cargo.toml` and `Cargo.lock` only) in a temporary worktree, runs `just test` there,
and pushes a signed commit "bump version to X" to the scratch `release` branch only. On
that exact commit, Test & Build runs, and so does a manual run of the Release workflow in
candidate mode: it tests, builds the Linux x86_64 musl archive, RPM and DEB and the macOS
x86_64 archive, packages and verifies the crate, and keeps all of it as artifacts with a
manifest of their SHA-256 sums, publishing nothing. When both pass, the script downloads
the manifest and every artifact and checks the commit, the version and every checksum,
then moves `develop`, `main` and the signed tag X together in one atomic push; the tag
message names the candidate run. The tag's Release run builds nothing: its guard checks
the tag (GitHub-verified signature, commit on `main`, version, Test & Build, the named
candidate run), the GitHub release gets exactly the manifest's files, and the crate goes
to crates.io, repackaged from the tag with the candidate's toolchain and uploaded only
when its checksum matches the manifest. The release is marked Latest only when its tag is
the highest promoted release at that moment.

If anything fails before the atomic push, nothing moved: no tag, `develop` and `main`
untouched. Re-run the failed jobs in GitHub (a waiting deploy picks the re-run up within
15 minutes), or fix it on `sandbox` and merge; then run `just deploy` again, which
resumes the same candidate and runs, replaces a stale one, or says there is nothing new
to release. A failed publish step in the tag's run is fixed with "Re-run failed jobs";
when the tagged workflow itself was wrong, fix it, release as usual, and run
`just release-republish X.Y.Z`. Recovery needs the candidate's artifacts, which GitHub
keeps for 90 days.

`RELEASE_POLL_SECONDS` (30), `RELEASE_CI_TIMEOUT` (3600, per attempt) and
`RELEASE_RERUN_WAIT` (900, after a failed attempt) are in seconds, and
`RELEASE_NO_WAIT=1` stops once the candidate is staged, or while CI still runs; a later
`just deploy` finishes. Releasing needs `just`, `jq`, `gh` (logged in), `cargo-edit`,
and a signing key that GitHub knows as a signing key, since commits and tags must be
signed.
