#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

cd ..
rm -rf conf.example
rsync --exclude='alter' --exclude='watch' -avL conf/ conf.example

cd conf.example

for i in alter watch; do
  mkdir -p $i
  touch $i/.keep
done

sed -i 's/\([0-9]\+\.\)\{2\}[0-9]\+/8.9.10/g' cluster/ol.yml
sed -i 's/[0-9a-f]\+:[0-9a-f]\+:[0-9a-f]\+/2abc:1234:5678/g' cluster/ol.yml

git add .

cd $DIR
direnv exec . ./conf_example.coffee
