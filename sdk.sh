#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

e() {
  direnv exec . $@
}

cd srv
bun x apint
cd ..
bun x protopkg srv sdk/js
