# Create building image based on rust
FROM --platform=$BUILDPLATFORM rust:1.74 as build

RUN mkdir -p /decentralml
RUN chown root:root /decentralml

RUN apt-get update
RUN apt install -y build-essential protobuf-compiler libclang-dev

# Clone the repository with substrate python client
RUN git clone https://github.com/livetreetech/DecentralML.git /tmp/
WORKDIR /tmp/substrate-node-decentralml

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
    useradd -m -s /bin/bash -N -u $NB_UID $NB_USER && \
    chmod g+w /etc/passwd && \
    echo "${NB_USER}    ALL=(ALL)    NOPASSWD:    ALL" >> /etc/sudoers

# Install Python and Python-pip
RUN apt-get install -y git python3 python3-pip

# Create folder specifically for DecentralML
RUN mkdir -p /decentralml
RUN chown root:root /decentralml

# Create assets folder to link at runtime
RUN mkdir -p /decentralml/assets
RUN chown root:root /decentralml/assets

# Create remote folder to simulate remote storage
RUN mkdir -p /decentralml/remote
RUN chown root:root /decentralml/remote

# Install poetry to install python client as a package
RUN pip install poetry
RUN poetry config virtualenvs.create false

# Copy content of substrate client repo from building image
COPY --from=build /tmp/substrate-client-decentralml /decentralml/substrate-client-decentralml
# Change the working directory to the client folder
WORKDIR /decentralml/substrate-client-decentralml
# Install substrate-client-decentral using poetryß
RUN poetry install

WORKDIR /

# Set shell
ENV SHELL=/bin/bash

# Set entrypoint for docker container based on this image
ENTRYPOINT [ "/bin/bash" ]