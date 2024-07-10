#!/bin/bash

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

rm -rf $SCRIPT_DIR/test_debug/gen/python/src

cargo test -- --nocapture

cd test_debug/gen/python/server || exit
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
mypy --strict src
