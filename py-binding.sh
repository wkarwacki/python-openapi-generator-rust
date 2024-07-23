#!/bin/bash

set -eox pipefail

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

cd /test_debug/dd2/python/server
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

cd "$SCRIPT_DIR"/plugin/py-binding
maturin develop
