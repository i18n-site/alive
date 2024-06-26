#!/usr/bin/env bash

DIR=$(realpath ${0%/*})
cd $DIR
set -ex

if ! command -v fly &>/dev/null; then
  curl -L https://fly.io/install.sh | sh
fi

./fly.secret.env.set.sh
./sh/rsync_conf.sh

cd ../conf
if [ -f "plugin.ol.yml" ]; then
  rm -f plugin.yml
  ln -s plugin.ol.yml plugin.yml
fi
cd ..

./plugin.sh
fly deploy
