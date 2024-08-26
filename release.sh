#!/bin/bash

set -eox pipefail

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

VERSION=$(yq -oy '.package.version' Cargo.toml)

trap "mv $SCRIPT_DIR/plugin/py-binding/Cargo.toml.bu $SCRIPT_DIR/plugin/py-binding/Cargo.toml; mv $SCRIPT_DIR/cli/py/pyproject.toml.bu $SCRIPT_DIR/cli/py/pyproject.toml; rm $SCRIPT_DIR/plugin/py-binding/README.md" EXIT

cp $SCRIPT_DIR/plugin/py-binding/Cargo.toml $SCRIPT_DIR/plugin/py-binding/Cargo.toml.bu
sed -i "s/{VERSION}/$VERSION/g" $SCRIPT_DIR/plugin/py-binding/Cargo.toml

cp $SCRIPT_DIR/cli/py/pyproject.toml $SCRIPT_DIR/cli/py/pyproject.toml.bu
sed -i "s/{VERSION}/$VERSION/g" $SCRIPT_DIR/cli/py/pyproject.toml

bash do_test.sh

cd plugin/py-binding
maturin upload

cd $SCRIPT_DIR/cli/py
twine upload dist/*
