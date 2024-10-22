#!/bin/bash

set -eox pipefail

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

pip install maturin

cp ../../README.md README.md

export PYENV_VERSION=3.10
maturin build

export PYENV_VERSION=3.11
maturin build
