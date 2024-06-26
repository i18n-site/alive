#!/usr/bin/env bash

DIR=$(realpath ${0%/*})
cd $DIR
set -ex

if [ ! $# -eq 2 ]; then
  echo "USAGE: $0 <tmpl> <project>"
  exit 1
fi

tmpl=$1
project=$2

cd ../$tmpl
cargo new --lib $project # this will add lib to workspace
rm -rf $project

cp -R ../tmpl/$tmpl $project

cd $DIR/../$1

rpl tmpl $project

git add .
