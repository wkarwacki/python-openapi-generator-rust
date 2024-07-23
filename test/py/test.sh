#!/bin/bash

set -eox pipefail

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

python -m venv .venv
source .venv/bin/activate

cd ../../wrapper/py
bash build_install.sh

cd $SCRIPT_DIR
trust help
