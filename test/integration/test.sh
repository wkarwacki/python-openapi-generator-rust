#!/bin/bash

set -eox pipefail

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

TESTS=openapi,openapi_fastapi python test.py
