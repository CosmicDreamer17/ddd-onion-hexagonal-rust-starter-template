#!/bin/bash
set -e

echo "Running verification..."

cd backend

echo "1. Checking Architectural Purity..."
if grep -r "infra" crates/domain; then
  echo "ERROR: Domain layer cannot import infra layer!"
  exit 1
fi

echo "2. Running Cargo fmt..."
cargo fmt --all -- --check

echo "3. Running Cargo clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "4. Running Cargo tests..."
cargo test --workspace

echo "Verification passed!"
