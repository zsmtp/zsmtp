default: test
  @just --list

# Test suite
test: clippy fmt-check unit-test
  @echo "✅ All tests passed!"

# Unit tests
unit-test:
  @echo "🧪 Running unit tests..."
  cargo test -- --nocapture

# Run tests with coverage
coverage:
  @echo "📊 Running tests with coverage..."
  cargo llvm-cov --all-features --workspace

# Linting
clippy:
  @echo "🔍 Running clippy..."
  cargo clippy --all-targets --all-features

# Formatting
fmt:
  @echo "🎨 Formatting code..."
  cargo fmt --all

# Verify formatting without modifying source files
fmt-check:
  @echo "🎨 Checking formatting..."
  cargo fmt --all -- --check

# Run benchmarks
bench:
  @echo "⚡ Running benchmarks..."
  cargo bench

# Build release version
build:
  @echo "🔨 Building release..."
  cargo build --release

# Build with musl for static linking
build-musl:
  @echo "🔨 Building with musl..."
  cargo build --release --target x86_64-unknown-linux-musl

# Update dependencies
update:
  @echo "⬆️  Updating dependencies..."
  cargo update

# Clean build artifacts
clean:
  @echo "🧹 Cleaning build artifacts..."
  cargo clean

# Get current version
version:
    @cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version'

# Releases run scripts/release: the version bump is staged on the scratch `release`
# branch, Test & Build and a candidate run of release.yml (every test, build and package,
# published nowhere) test that exact commit, and only then do develop, main and the
# signed tag move together in one atomic push; the tag's run publishes exactly what the
# candidate run built. Every deploy recipe is idempotent: rerunning it resumes the staged
# candidate, or says there is nothing left to release. See README "Releasing".

# Deploy: stage a patch bump, test and package it, then release it
deploy:
    @scripts/release deploy patch

# Deploy with minor version bump
deploy-minor:
    @scripts/release deploy minor

# Deploy with major version bump
deploy-major:
    @scripts/release deploy major

# Release develop's version as is when it has no tag yet (no new bump)
deploy-current:
    @scripts/release deploy current

# Show where a release stands: develop, main, the staged candidate and its runs
release-status:
    @scripts/release status

# Check everything a release needs; changes nothing that lasts
release-preflight:
    @scripts/release preflight

# Publish an existing release tag again if its own run cannot (recovery run on main)
release-republish version:
    @scripts/release republish {{version}}

# Apply branch protection (main requires "CI OK") and the rule that release tags never move
protect-branches:
    @scripts/release protect

# Build and package the current branch like a release candidate; releases nothing, no tag
release-dry-run:
    @scripts/release dry-run

# Check for security vulnerabilities
audit:
  @echo "🔒 Checking for security vulnerabilities..."
  cargo audit

# Check dependency licenses
deny:
  @echo "📜 Checking dependency licenses..."
  cargo deny check

# Full CI check (what runs in CI)
ci: clippy fmt test audit deny
  @echo "✅ All CI checks passed!"

# Build RPM package
build-rpm: build
  @echo "📦 Building RPM package..."
  cargo generate-rpm

# Build DEB package
build-deb: build
  @echo "📦 Building DEB package..."
  cargo deb

# Build all packages
build-packages: build-rpm build-deb
  @echo "✅ All packages built!"

# Show documentation
doc:
  @echo "📚 Building and opening documentation..."
  cargo doc --open --no-deps

# Check outdated dependencies
outdated:
  @echo "📅 Checking for outdated dependencies..."
  cargo outdated --root-deps-only
