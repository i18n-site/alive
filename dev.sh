#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

if [ ${#1} -eq 0 ]; then
  if [ -f ".dev" ]; then
    arg=$(cat .dev)
  else
    echo "❯ $0 项目名"
    exit 1
  fi
else
  echo $@ >.dev
  arg=$@
fi

source ./sh/pid.sh
set -ex
cd conf
rm -f plugin.yml
ln -s plugin.dev.yml plugin.yml
cd ..
if [ ! -f "alive_plugin/src/alter.rs" ]; then
  ./plugin.sh
else
  file_path="conf/plugin.yml"
  temp_file=".conf.plugin.yml.md5"

  current_md5=$(md5sum "$file_path" | awk '{ print $1 }')

  if [ -f "$temp_file" ]; then
    previous_md5=$(cat "$temp_file")

    if [ "$current_md5" != "$previous_md5" ]; then
      ./plugin.sh
    fi
  else
    ./plugin.sh
  fi
  echo "$current_md5" >"$temp_file"
fi

# if ! [ -x "$(command -v dasel)" ]; then
#   go install github.com/tomwright/dasel/v2/cmd/dasel@master
# fi
[[ -d target ]] && cargo sweep --time 30 && cargo sweep --installed

exec watchexec \
  --shell=none \
  --project-origin . -w . \
  --exts rs,toml \
  -r \
  -- ./run.sh $arg
