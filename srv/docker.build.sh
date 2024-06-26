#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

./sh/rsync_conf.sh

name=alive
cd ..

docker build -t $name .

docker tag $name $name:latest
