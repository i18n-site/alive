#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

set -e

GZIP_HEADER="\x1f\x8b\x08\x00\x00\x00\x00\x00"

export ENV=$( (printf $GZIP_HEADER && echo $DIRENV_DIFF | base64 -d) | gzip -dc 2>/dev/null || true)

bun x direnv_dump >../../.env
