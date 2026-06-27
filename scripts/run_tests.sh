#!/usr/bin/env bash
# Rust Quest test harness (Unix/macOS/CI)
# LEARN: run ./scripts/run_tests.sh from the repo root.

set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo ""
echo "STEP 1 — cargo fmt --check (Rust style gate)"
cargo fmt --all -- --check

echo ""
echo "STEP 2 — cargo check (fast compile sanity)"
cargo check

echo ""
echo "STEP 3 — cargo clippy (linter, warnings as errors)"
cargo clippy --all-targets --all-features -- -D warnings

echo ""
echo "STEP 4 — cargo test (unit + integration + doc tests)"
cargo test --all-targets

echo ""
echo "STEP 5 — cargo build --release (release build check)"
cargo build --release

echo ""
echo "All steps passed!"
