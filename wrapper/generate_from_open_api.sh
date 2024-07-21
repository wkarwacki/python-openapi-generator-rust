#!/bin/bash

TRUST_IMAGE=trust:latest

set -eox pipefail

TMP_DIR=$(mktemp -d)
cd "$TMP_DIR" || exit

trap 'docker rm -f trust; rm -rf "$TMP_DIR/trust/build"' EXIT

INPUT_FILE="$(basename $1)"
INPUT_DIR="$(dirname "$1")"
OUTPUT_DIR=$2


rm -rf $TMP_DIR/trust/build/api
mkdir -p $TMP_DIR/trust/build/api

docker run --name trust -v $INPUT_DIR:/run/trust/openapi $TRUST_IMAGE from-open-api /run/trust/openapi/$INPUT_FILE /run/trust/api -l=tag
docker cp trust:/run/trust/api/. $TMP_DIR/trust/build
docker rm -f trust


rm -rf $TMP_DIR/src/trust
mkdir -p $TMP_DIR/src/trust

for yml_path in "$TMP_DIR/trust/build"/*.yml;
do
  docker run --name trust -v $TMP_DIR/trust/build:/run/trust/build $TRUST_IMAGE generate python client "/run/trust/build/$(basename "$yml_path")" /run/trust/out
  docker cp trust:/run/trust/out/. $OUTPUT_DIR
  docker rm -f trust
done
