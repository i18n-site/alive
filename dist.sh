#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

if [ -z "$1" ]; then
  echo "USAGE : $0 commit msg"
  exit 1
fi
set -ex

./sh/conf_example.sh
bun x mdt .
git add .
gme $@
git push github main -f
