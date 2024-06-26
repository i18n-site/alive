#!/usr/bin/env bash

export RUST_LOG=debug,process_wrap=warn,supervisor=warn,hyper=warn,reqwest=warn,rustls=warn,h2=warn,tower=warn,h3=warn,quinn_udp=warn,quinn_proto=warn,watchexec=warn,globset=warn,hickory_proto=warn,hickory_resolver=warn,aws_smithy_runtime=warn,aws_sdk_s3=warn,fred=warn
export RUST_BACKTRACE=short

cd $(dirname $(realpath $BASH_SOURCE))/conf
for i in $(find . -name "*.sh"); do
  set -o allexport
  . $i
  set +o allexport
done
