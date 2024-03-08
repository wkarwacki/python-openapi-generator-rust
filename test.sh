#!/bin/bash

rm -rf test_debug/gen/kotlin/src
mkdir -p test_debug/gen/kotlin/src/main/kotlin/adt
mkdir -p test_debug/gen/kotlin/src/main/kotlin/mix
mkdir -p test_debug/gen/kotlin/src/main/kotlin/mixofmix
mkdir -p test_debug/gen/kotlin/src/main/kotlin/typeparams
mkdir -p test_debug/gen/kotlin/src/main/kotlin/types
mkdir -p test_debug/gen/kotlin/src/main/kotlin/vars

rm -rf /home/wkarwacki/code/trust/test_debug/gen/scala/target/scala-3.4.2/src_managed/trust
mkdir -p /home/wkarwacki/code/trust/test_debug/gen/scala/target/scala-3.4.2/src_managed/trust/scala/adt
mkdir -p /home/wkarwacki/code/trust/test_debug/gen/scala/target/scala-3.4.2/src_managed/trust/scala/mix
mkdir -p /home/wkarwacki/code/trust/test_debug/gen/scala/target/scala-3.4.2/src_managed/trust/scala/mixofmix
mkdir -p /home/wkarwacki/code/trust/test_debug/gen/scala/target/scala-3.4.2/src_managed/trust/scala/typeparams
mkdir -p /home/wkarwacki/code/trust/test_debug/gen/scala/target/scala-3.4.2/src_managed/trust/scala/types
mkdir -p /home/wkarwacki/code/trust/test_debug/gen/scala/target/scala-3.4.2/src_managed/trust/scala/vars

rm -rf /home/wkarwacki/code/trust/test_debug/gen/python/src/trust
rm -rf /home/wkarwacki/code/trust/test_debug/gen/python/.mypy_cache
mkdir -p /home/wkarwacki/code/trust/test_debug/gen/python/src/trust/adt
mkdir -p /home/wkarwacki/code/trust/test_debug/gen/python/src/trust/mix
mkdir -p /home/wkarwacki/code/trust/test_debug/gen/python/src/trust/mixofmix
mkdir -p /home/wkarwacki/code/trust/test_debug/gen/python/src/trust/typeparams
mkdir -p /home/wkarwacki/code/trust/test_debug/gen/python/src/trust/types
mkdir -p /home/wkarwacki/code/trust/test_debug/gen/python/src/trust/vars

rm -rf /home/wkarwacki/code/trust/test_debug/gen/typescript/src/trust
rm -rf /home/wkarwacki/code/trust/test_debug/gen/typescript/.mypy_cache
mkdir -p /home/wkarwacki/code/trust/test_debug/gen/typescript/src/trust/adt
mkdir -p /home/wkarwacki/code/trust/test_debug/gen/typescript/src/trust/mix
mkdir -p /home/wkarwacki/code/trust/test_debug/gen/typescript/src/trust/mixofmix
mkdir -p /home/wkarwacki/code/trust/test_debug/gen/typescript/src/trust/typeparams
mkdir -p /home/wkarwacki/code/trust/test_debug/gen/typescript/src/trust/types
mkdir -p /home/wkarwacki/code/trust/test_debug/gen/typescript/src/trust/vars

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
