#!/bin/bash

set -eox pipefail

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

bash test.sh

cd plugin/py-binding
maturin upload

cd $SCRIPT_DIR/cli/py
twine upload dist/*
