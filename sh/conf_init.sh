#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

cd ..

if [ ! -f "conf/plugin.yml" ]; then
  rm -rf conf/*
  mkdir -p ../conf/alive
  rsync -avz conf.example/ conf
fi

cd $DIR
direnv exec . ./conf_init.coffee
