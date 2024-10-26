#!/bin/bash

set -eox pipefail

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

rm -rf .venv
python -m venv .venv
source .venv/bin/activate

cd ../../plugin/py-binding
bash install.sh

cd $SCRIPT_DIR/../../cli/py
bash build.sh

pip uninstall -y trustspeccli
pip install dist/trustspeccli-*-py3-none-any.whl

cd $SCRIPT_DIR
trust -V
