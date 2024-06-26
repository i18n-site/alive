#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex

if [ ! -d "node_modules" ]; then
  ni
fi

../srv/ssl/up.sh

# 国际化
# ./sh/plugin.sh
bun x plugin
rm -rf node_modules/.vite vite.config.js.timestamp-*.mjs
direnv exec . ./sh/svg.var.coffee
cd src
rm -rf conf.js
ln -s ../conf/dev.js conf.js
cd $DIR

# link() {
#   sleep 1
#   wasm=__bg.wasm
#   deps=node_modules/.vite/deps/$wasm
#   if [ ! -s "$deps" ]; then
#     mkdir -p $(dirname $deps)
#     ln -s ../../@3-/vite/$wasm $deps
#   fi
# }
# link &
# cd $DIR
exec direnv exec . ./sh/dev.sh
