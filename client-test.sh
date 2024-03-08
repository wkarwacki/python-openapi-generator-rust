#!/bin/bash

set -eox pipefail

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

GENERATOR=python
ROLE=client
TRUST_SPEC_DIR=$SCRIPT_DIR/test_debug/dd/trust

rm -rf $TRUST_SPEC_DIR
mkdir -p $TRUST_SPEC_DIR

cargo run from-open-api $SCRIPT_DIR/src/test/dd/api.yml $TRUST_SPEC_DIR

#OUTPUT_DIR=$SCRIPT_DIR/test_debug/dd/$GENERATOR/target/scala-3.4.2/src_managed/trust/scala
OUTPUT_DIR=$SCRIPT_DIR/test_debug/dd/$GENERATOR/$ROLE/src/trust

rm -rf $OUTPUT_DIR
mkdir -p $OUTPUT_DIR

cargo run generate $GENERATOR $ROLE $TRUST_SPEC_DIR/analytic.yml $OUTPUT_DIR
cargo run generate $GENERATOR $ROLE $TRUST_SPEC_DIR/api.yml $OUTPUT_DIR
cargo run generate $GENERATOR $ROLE $TRUST_SPEC_DIR/dev.yml $OUTPUT_DIR
cargo run generate $GENERATOR $ROLE $TRUST_SPEC_DIR/entity-relation.yml $OUTPUT_DIR
cargo run generate $GENERATOR $ROLE $TRUST_SPEC_DIR/experiment.yml $OUTPUT_DIR
cargo run generate $GENERATOR $ROLE $TRUST_SPEC_DIR/feature.yml $OUTPUT_DIR
cargo run generate $GENERATOR $ROLE $TRUST_SPEC_DIR/log.yml $OUTPUT_DIR
cargo run generate $GENERATOR $ROLE $TRUST_SPEC_DIR/model.yml $OUTPUT_DIR
cargo run generate $GENERATOR $ROLE $TRUST_SPEC_DIR/prediction.yml $OUTPUT_DIR
cargo run generate $GENERATOR $ROLE $TRUST_SPEC_DIR/search.yml $OUTPUT_DIR
cargo run generate $GENERATOR $ROLE $TRUST_SPEC_DIR/table.yml $OUTPUT_DIR
cargo run generate $GENERATOR $ROLE $TRUST_SPEC_DIR/task.yml $OUTPUT_DIR

cd $OUTPUT_DIR/../.. && SERVER_BASE_URL=http://localhost:8000 ./run.sh

#sbt compile

#gradle ktlintformat
#gradle build
