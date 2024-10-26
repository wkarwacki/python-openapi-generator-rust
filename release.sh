#!/bin/bash

set -eox pipefail

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

export VERSION=$(yq -oy '.package.version' Cargo.toml)

bash do_release.sh

cd $SCRIPT_DIR
git add .
git commit -m "release TrustSpecCli $VERSION"
git push origin master
