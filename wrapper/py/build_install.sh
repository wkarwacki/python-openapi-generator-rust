#!/bin/bash

set -eox pipefail

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

cd ../../plugin/py-binding
pip install maturin
maturin develop

cd $SCRIPT_DIR
bash install.sh
