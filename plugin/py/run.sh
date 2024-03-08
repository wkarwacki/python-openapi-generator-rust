#!/bin/bash

set -eox pipefail

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")

python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

python $SCRIPT_DIR/main.py "$@"
