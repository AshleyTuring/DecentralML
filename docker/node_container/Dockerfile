# Create building image based on rust
FROM --platform=$BUILDPLATFORM rust:1.74 as build

# Create folder for bulding node
RUN mkdir -p /decentralml
RUN chown root:root /decentralml

# Update image and install building tools
RUN apt-get update
RUN apt install -y build-essential protobuf-compiler libclang-dev

# Clone the repo for DecentralML
RUN git clone https://github.com/livetreetech/DecentralML.git /decentralml
WORKDIR /decentralml/substrate-node-decentralml

# Build DecentralML node in rust
RUN cargo build --package node-decentralml --release

# Setup actual image based on Ubuntu
FROM --platform=$BUILDPLATFORM ubuntu:22.04

# Setup user for future application
ARG NB_USER="decentralml"
ARG NB_UID="1000"
ARG NB_GID="100"
ARG TARGETPLATFORM
RUN echo "I'm building for $TARGETPLATFORM"

# Update Ubuntu
RUN \
    apt-get update && \
    apt-get install -y sudo && \
    useradd -m -s /bin/bash -N -u $NB_UID $NB_USER && \
    chmod g+w /etc/passwd && \
    echo "${NB_USER}    ALL=(ALL)    NOPASSWD:    ALL" >> /etc/sudoers

# Create folder for DecentralML
RUN mkdir -p /decentralml
RUN chown root:root /decentralml

# Copy built project from build image
COPY --from=build /decentralml/substrate-node-decentralml/target/release/node-decentralml /decentralml/node-decentralml
# Copy launch script from building folder
COPY launch_node.sh /decentralml/launch_node.sh
# Make launch script executable
RUN chmod +x /decentralml/launch_node.sh

# Set shell
ENV SHELL=/bin/bash

# Set entrypoint for docker container based on this image to aumatically launch a node
ENTRYPOINT [ "/decentralml/launch_node.sh" ]