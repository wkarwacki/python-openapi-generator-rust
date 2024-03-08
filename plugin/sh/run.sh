#!/bin/bash

set -eox pipefail

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

INPUT=$1
CONFIG=$2
TEMPLATES=$3
OUTPUT=$4

INPUT_FILE=$(basename $INPUT)

docker run --name trust \
  -v $INPUT/..:/run/trust/api \
  -v $CONFIG:/run/trust/cfg.yml \
  -v $TEMPLATES:/usr/src/trust/src/trust \
  trust python /run/trust/api/$INPUT_FILE /run/trust/out /run/trust/cfg.yml
docker cp trust:/run/trust/out/. $OUTPUT
docker rm -f trust
