#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

name=$1
# name=${1:-rsrv}
# name=$(dasel package.name -f Cargo.toml)
# name=${name//\'/}

exe=./target/debug/$name
rm -rf $exe

# cargo build --all-features -p $name

sh=$name/test.sh
if [ -f "$sh" ]; then
  direnv exec . ./$sh
else
  cargo build -p $(basename $name)
fi
