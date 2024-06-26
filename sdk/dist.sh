#!/usr/bin/env bash

DIR=$(realpath ${0%/*})
cd $DIR
set -ex

cd js
exec dist.coffee
