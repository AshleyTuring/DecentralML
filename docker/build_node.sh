#!/bin/bash
export DOCKER_BUILDKIT=1
export BUILDKIT_PROGRESS=plain
export DOCKER_DEFAULT_PLATFORM=$BUILDPLATFORM

docker build --no-cache --pull -t decentralml_node:latest ./node_container/