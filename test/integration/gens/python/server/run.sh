#!/bin/bash

set -eox pipefail

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

source .venv/bin/activate

autoflake --recursive --in-place --remove-all-unused-imports src/trust

cd src && python -m app.main
