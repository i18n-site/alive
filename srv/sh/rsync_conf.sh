#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

cd ../..
rm -rf tmp
mkdir -p tmp

rsync -avL --include="*/" --include="*.yml" --exclude="*" conf/ tmp/conf

cd tmp/conf
rm -f plugin.*.yml plugin.yml
