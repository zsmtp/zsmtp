#!/usr/bin/env bash
# release-flow-adapter: 1
# zsmtp project adapter: the framework's generic Cargo adapter plus the crate
# artifact that the crates-io publisher republishes byte-for-byte.
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cargo_adapter="${here}/../scripts/adapters/cargo.sh"
linux_target="x86_64-unknown-linux-musl"
export RELEASE_FLOW_CARGO_TARGETS="${linux_target}:ubuntu-24.04 x86_64-apple-darwin:macos-latest"

command="${1:-}"
version="${2:-}"

case "${command}" in
  inventory)
    # The crate is built in the Linux leg only, so it is declared once.
    "${cargo_adapter}" inventory "${version}" | jq --arg v "${version}" --arg t "${linux_target}" \
      '.artifacts += [{name: ("zsmtp-" + $v + ".crate"), target: $t}]'
    ;;
  package)
    outdir="${3:-}"
    "${cargo_adapter}" package "${version}" "${outdir}"
    if [[ -z "${RELEASE_FLOW_TARGET:-}" || "${RELEASE_FLOW_TARGET}" == "${linux_target}" ]]; then
      # --no-verify: the candidate's verify step already built and tested this commit.
      cargo package --no-verify --locked >&2
      cp "target/package/zsmtp-${version}.crate" "${outdir}/"
    fi
    ;;
  *)
    exec "${cargo_adapter}" "$@"
    ;;
esac
