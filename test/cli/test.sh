#!/bin/bash

set -eox pipefail

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

python -m venv .venv
source .venv/bin/activate

cd ../../cli/py
bash build.sh
pip install --force dist/trustspecgen-*-py3-none-any.whl

cd $SCRIPT_DIR
trust help
