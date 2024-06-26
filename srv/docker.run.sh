#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

./sh/rsync_conf.sh
cd ..
docker-compose down
docker-compose build
docker-compose up
