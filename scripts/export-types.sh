#!/bin/bash
set -e

echo 'Exporting TypeScript bindings...'
cd backend
cargo test -p domain

mkdir -p ../frontend/types/generated
cp crates/domain/bindings/*.ts ../frontend/types/generated/
echo 'Types exported to frontend/types/generated/'
