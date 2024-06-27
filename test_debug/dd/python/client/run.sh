#!/bin/bash

set -eox pipefail

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

autoflake --recursive --in-place --remove-all-unused-imports src/trust
cd src && python -m app.main
