#!/usr/bin/env bash

set -euo pipefail

THIS_DIR="$(cd "$(dirname "$0")" && pwd)"
WORK_DIR="${THIS_DIR}/../../dot-pr-review"
BIN="${WORK_DIR}/target/release/dot-pr-review"

# Build if the binary does not exist
if [[ ! -x "${BIN}" ]]; then
    echo "Binary not found — building release version..."
    cd "${WORK_DIR}"
    cargo build -r
fi

# Run the tool with all arguments passed to this script
exec "${BIN}" "$@"
