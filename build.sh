#!/bin/bash

#cargo build --release --target x86_64-unknown-linux-musl

# saw it recommended here https://github.com/sfackler/rust-openssl/issues/603
# source here: https://github.com/clux/muslrust
# can't run it other than root, it can't find cargo

sudo echo "Build a new docker image and container"

docker run  --rm -t \
	-v $PWD:/volume \
	clux/muslrust \
	cargo build --release

sudo chown -R ovidiu:ovidiu target
strip target/x86_64-unknown-linux-musl/release/inform

NAME=inform
ID=$(cargo pkgid | awk -F '#' '{ print $2 }')

docker stop $NAME
docker rm $NAME

docker image rm $NAME:$ID

docker build --tag $NAME:$ID .

docker run -p 7878:7878 --name $NAME -d --restart unless-stopped --user $(id -u nobody):$(id -g nobody) $NAME:$ID
