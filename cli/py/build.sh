#!/bin/bash

set -eox pipefail

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

cd $SCRIPT_DIR
rm -rf dist
pip install poetry
poetry build
