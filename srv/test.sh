#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

RUSTFLAGS="--cfg tokio_unstable $RUSTFLAGS" cargo run --features tokio_console -- -c $DIR/../conf --port 8800
