#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex
source env.sh
if ! command -v sponge &>/dev/null; then
  case $(uname -s) in
  Linux*)
    apt-get install -y moreutils
    ;;
  Darwin*)
    brew install moreutils
    ;;
  esac
fi

export NODE_ENV=production

./svg.var.coffee

cd $(dirname $DIR)

bun x plugin
bun x vite build

cd $DIR
./public.js.sh
