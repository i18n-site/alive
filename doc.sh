#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

if [ -z "$1" ]; then
  echo "USAGE : $0 project_name"
  exit 1
else
  export PROJECT=$1
fi

set -ex

direnv exec . cargo doc --open --all-features -p $PROJECT
