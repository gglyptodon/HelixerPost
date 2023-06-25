FROM rust:1-slim-bookworm
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install git libhdf5-dev

RUN bash