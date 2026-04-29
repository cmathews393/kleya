# [Choice] Debian OS version (use bookworm, or bullseye on local arm64/Apple Silicon): bookworm, buster, bullseye
ARG VARIANT="trixie"
FROM rust:1-${VARIANT}




RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends curl git build-essential cmake pkg-config libssl-dev


RUN git clone https://github.com/cmathews393/kleya.git /kleya
WORKDIR /kleya