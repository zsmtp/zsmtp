import '.justfiles/deploy.just'

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

# Releases come from the release framework (nbari/release); recipes live in
# .justfiles/deploy.just. Configuration: .release-flow.json; adapter: tools/adapter.sh.
# See README "Releasing".

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
