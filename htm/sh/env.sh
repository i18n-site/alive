DIR=$(pwd)
# env_sh() {
#   cd "$(dirname $(realpath ${BASH_SOURCE[0]}))"/../../conf/conn
#   local i
#   for i in $@; do
#     set -o allexport
#     source "$i".sh
#     set +o allexport
#   done
#
#   cd $DIR
#   unset -f env_sh
# }
#
# env_sh web web.cdn cf

# 检测 clashx 是否可用, 如果可用就用代理上网
if command -v nc &>/dev/null; then
  nc -z -w 1 127.0.0.1 7890 &&
    export https_proxy=http://127.0.0.1:7890 &&
    export NODE_TLS_REJECT_UNAUTHORIZED=0
fi
