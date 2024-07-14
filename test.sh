#!/bin/bash

set -eox pipefail

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

rm -rf  test/default
mkdir -p test/default/spec

cargo test -- --nocapture

TESTS=openapi,openapi_fastapi python test/integration/test.py
