# Rust Quest test harness (Windows)
# LEARN: run this script from anywhere — it cd's to the repo root.

$ErrorActionPreference = 'Stop'
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host ""
Write-Host "STEP 1 - cargo fmt --check (Rust style gate)"
cargo fmt --all -- --check

Write-Host ""
Write-Host "STEP 2 - cargo check (fast compile sanity)"
cargo check

Write-Host ""
Write-Host "STEP 3 - cargo clippy (linter, warnings as errors)"
cargo clippy --all-targets --all-features -- -D warnings

Write-Host ""
Write-Host "STEP 4 - cargo test (unit + integration + doc tests)"
cargo test --all-targets

Write-Host ""
Write-Host "STEP 5 - cargo build --release (release build check)"
cargo build --release

Write-Host ""
Write-Host "All steps passed!"
