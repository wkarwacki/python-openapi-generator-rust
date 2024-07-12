#!/bin/bash

set -eox pipefail

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

cargo +nightly fmt
cargo fix --lib -p trust --allow-dirty
