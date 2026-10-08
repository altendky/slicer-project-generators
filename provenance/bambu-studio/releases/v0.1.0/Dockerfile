FROM ubuntu:24.04@sha256:4fbb8e6a8395de5a7550b33509421a2bafbc0aab6c06ba2cef9ebffbc7092d90

RUN apt-get -o Acquire::https::Verify-Peer=false update --snapshot 20260731T230600Z \
    && DEBIAN_FRONTEND=noninteractive apt-get \
        -o Acquire::https::Verify-Peer=false install --yes --no-install-recommends \
        --snapshot 20260731T230600Z build-essential ca-certificates \
    && rm -rf /var/lib/apt/lists/*

ENV PATH="/rust/bin:${PATH}" CARGO_HOME=/build/cargo RUSTC=/rust/bin/rustc \
    CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/build/target
WORKDIR /src
