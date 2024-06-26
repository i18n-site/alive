#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex

cd ../public

shpublic=$DIR/public
bun x cep -c $shpublic

minify() {
  esbuild $shpublic/$1Worker.js \
    --minify \
    --bundle \
    --external:onconnect \
    --format=iife |
    sed 's/^.\{6\}//; s/.\{6\}$//' >$2.js
}

minify service s
minify share w
# rm sw.js

cd ..

# bun x vite build

# cd dist
