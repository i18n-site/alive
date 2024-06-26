#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

direnv exec . ./sh/plugin.coffee
rm -f .conf.plugin.yml.md5
