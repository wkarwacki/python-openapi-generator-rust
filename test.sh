#!/bin/bash

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

rm -rf test_debug/gen/kotlin/src
mkdir -p test_debug/gen/kotlin/src/main/kotlin/adt
mkdir -p test_debug/gen/kotlin/src/main/kotlin/mix
mkdir -p test_debug/gen/kotlin/src/main/kotlin/mixofmix
mkdir -p test_debug/gen/kotlin/src/main/kotlin/typeparams
mkdir -p test_debug/gen/kotlin/src/main/kotlin/types
mkdir -p test_debug/gen/kotlin/src/main/kotlin/vars

rm -rf $SCRIPT_DIR/test_debug/gen/scala/target/scala-3.4.2/src_managed/trust
mkdir -p $SCRIPT_DIR/test_debug/gen/scala/target/scala-3.4.2/src_managed/trust/scala/adt
mkdir -p $SCRIPT_DIR/test_debug/gen/scala/target/scala-3.4.2/src_managed/trust/scala/mix
mkdir -p $SCRIPT_DIR/test_debug/gen/scala/target/scala-3.4.2/src_managed/trust/scala/mixofmix
mkdir -p $SCRIPT_DIR/test_debug/gen/scala/target/scala-3.4.2/src_managed/trust/scala/typeparams
mkdir -p $SCRIPT_DIR/test_debug/gen/scala/target/scala-3.4.2/src_managed/trust/scala/types
mkdir -p $SCRIPT_DIR/test_debug/gen/scala/target/scala-3.4.2/src_managed/trust/scala/vars

rm -rf $SCRIPT_DIR/test_debug/gen/python/src
rm -rf $SCRIPT_DIR/test_debug/gen/python/.mypy_cache
mkdir -p $SCRIPT_DIR/test_debug/gen/python/src/adt
mkdir -p $SCRIPT_DIR/test_debug/gen/python/src/mix
mkdir -p $SCRIPT_DIR/test_debug/gen/python/src/mixofmix
mkdir -p $SCRIPT_DIR/test_debug/gen/python/src/typeparams
mkdir -p $SCRIPT_DIR/test_debug/gen/python/src/types
mkdir -p $SCRIPT_DIR/test_debug/gen/python/src/vars

rm -rf $SCRIPT_DIR/test_debug/gen/typescript/src
rm -rf $SCRIPT_DIR/test_debug/gen/typescript/.mypy_cache
mkdir -p $SCRIPT_DIR/test_debug/gen/typescript/src/adt
mkdir -p $SCRIPT_DIR/test_debug/gen/typescript/src/mix
mkdir -p $SCRIPT_DIR/test_debug/gen/typescript/src/mixofmix
mkdir -p $SCRIPT_DIR/test_debug/gen/typescript/src/typeparams
mkdir -p $SCRIPT_DIR/test_debug/gen/typescript/src/types
mkdir -p $SCRIPT_DIR/test_debug/gen/typescript/src/vars

cargo test -- --nocapture

#cd test_debug/gen/kotlin || exit
#gradle ktlintformat
#gradle build

#cd test_debug/gen/scala || exit
#sbt compile

cd test_debug/gen/python || exit
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
mypy --strict src

#cd test_debug/gen/typescript || exit
#npm install
