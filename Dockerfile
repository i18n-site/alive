FROM ubuntu as build

WORKDIR /app

ENV PIP_ROOT_USER_ACTION=ignore

# RUN apt-get update &&\
#   apt-get install -y python3-pip &&\
#   pip install apt-smart &&\
#   apt-smart -a 

RUN \
apt-get update &&\
apt-get install -y \
ca-certificates protobuf-compiler curl bash build-essential libssl-dev pkg-config mold clang &&\
update-ca-certificates

SHELL [ "/bin/bash", "-c" ]

ENV SHELL=/bin/bash

ENV CARGO_HOME=/opt/rust
ENV RUSTUP_HOME=/opt/rust

RUN curl https://sh.rustup.rs -sSf | \
sh -s -- -y --no-modify-path --default-toolchain none &&\
source $CARGO_HOME/env &&\
rustup toolchain install nightly --profile minimal


ADD ./sh/cpso.sh .
ADD Cargo.toml .
ADD alive alive
ADD alive_alter alive_alter
ADD alive_api alive_api
ADD alive_plugin alive_plugin
ADD alive_watch alive_watch
ADD alter alter
ADD srv srv
ADD watch watch


RUN \
source $CARGO_HOME/env &&\
mkdir -p out &&\
TARGET=$(rustc -vV | sed -n 's|host: ||p') &&\
export RUSTFLAGS="--cfg reqwest_unstable -Ctarget-feature=+crt-static $RUSTFLAGS" &&\
cargo build \
  --release -p srv \
  --out-dir out \
  -Z unstable-options \
  --target=$TARGET &&\
mv out/* m &&\
./cpso.sh m

# FROM ubuntu
FROM scratch

ENV LD_LIBRARY_PATH=/lib
COPY --from=build /so/ lib/

WORKDIR /
COPY --from=build /etc/ssl/certs /etc/ssl/certs
COPY --from=build /app/m .
ADD tmp/conf conf

ENTRYPOINT [ "/m","-c","/conf", "--port","5123" ]
