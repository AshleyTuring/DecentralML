#!/bin/bash
export DOCKER_BUILDKIT=1
export BUILDKIT_PROGRESS=plain
export DOCKER_DEFAULT_PLATFORM=$BUILDPLATFORM

# cargo clean --target-dir ../substrate-node-decentralml/
# cargo build --manifest-path ../substrate-node-decentralml/Cargo.toml --package node-decentralml --release

docker build --no-cache --pull -t decentralml_node:latest ./node_container/