FROM --platform=$BUILDPLATFORM rust:1.74 as build

RUN mkdir -p /decentralml
RUN chown root:root /decentralml

RUN apt-get update
RUN apt install -y build-essential protobuf-compiler libclang-dev

RUN git clone https://github.com/livetreetech/DecentralML.git /tmp
WORKDIR /tmp/substrate-node-decentralml

FROM --platform=$BUILDPLATFORM ubuntu:22.04

ARG NB_USER="decentralml"
ARG NB_UID="1000"
ARG NB_GID="100"
ARG TARGETPLATFORM
RUN echo "I'm building for $TARGETPLATFORM"

RUN \
    apt-get update && \
    apt-get install -y sudo && \
    useradd -m -s /bin/bash -N -u $NB_UID $NB_USER && \
    chmod g+w /etc/passwd && \
    echo "${NB_USER}    ALL=(ALL)    NOPASSWD:    ALL" >> /etc/sudoers

RUN apt-get install -y git python3 python3-pip

RUN git clone https://github.com/livetreetech/DecentralML.git /tmp/

RUN mkdir -p /decentralml
RUN chown root:root /decentralml

RUN mkdir -p /decentralml/assets
RUN chown root:root /decentralml/assets

COPY --from=build /tmp/substrate-client-decentralml /decentralml/substrate-client-decentralml
RUN pip install -r /decentralml/substrate-client-decentralml/requirements.txt

ENV SHELL=/bin/bash

ENTRYPOINT [ "/bin/bash" ]