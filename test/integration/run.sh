#!/bin/bash

set -eox pipefail

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR" || exit

pm2 delete trust-server || true

trap 'pm2 delete trust-server' EXIT

rm -rf log
mkdir -p log
pm2 start $1 --name trust-server --log log/trust-server -f

SERVER_BASE_URL=http://localhost:8000

while true; do
    response=$(curl -s -o /dev/null -w "%{http_code}" $SERVER_BASE_URL/health || true)
    if [[ $response -eq 204 ]]; then
        echo "Server is healthy (HTTP 204)"
        break
    else
        echo "Server is not yet healthy (HTTP $response)"
        sleep 1
    fi
done


SERVER_BASE_URL=$SERVER_BASE_URL $2
