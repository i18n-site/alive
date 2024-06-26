#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

LI=$(flyctl secrets list | awk '{print $1}' | tr '\n' ' ')

flyctl secrets unset $LI
