#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

set -ex
./sh/env.sh

cd ..

bash -c "flyctl secrets set $(cat ./.env | tr '\n' ' ')"
