#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex

rm -rf dist

node_version=$(node -v)
node_version=${node_version:1} # remove 'v' prefix
IFS='.' read -ra ADDR <<<"$node_version"
major_version=${ADDR[0]}

if ((major_version < 21)); then
  echo "node.js < 21"
  curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/master/install.sh | bash
  export NVM_DIR="$HOME/.nvm"
  . $NVM_DIR/nvm.sh
  nvm install node
  nvm use node
fi

if [ ! -d "node_modules" ]; then
  pnpm i
fi

if ! command -v direnv &>/dev/null; then
  curl -sfL https://direnv.net/install.sh | bash
  direnv allow
fi

cd src
ln -s ../conf/ol.js conf.js
cd ..

direnv exec . ./sh/build.sh

cd dist
mv index.htm index.html

#rm -rf "../dist/.18"
#./cdn.sh

# bun x xxai-dist
# git add -u
# git commit -m'dist' || true
# git push
#
# if [ -v GIT_PAGE ]; then
#   mkdir -p dist.git
#   cd dist.git
#   gitdir=$(basename $GIT_PAGE)
#   gitdir=${gitdir%.git}
#   gitdir="${gitdir#*:}"
#   rm -rf $gitdir
#   cp -R ../dist $gitdir
#   cd $gitdir
#   git init
#   git add .
#   git commit -m'dist'
#   git branch -M main
#   git remote add origin $GIT_PAGE
#   git push -f -u origin main
# fi
