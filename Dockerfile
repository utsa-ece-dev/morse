FROM rust:slim-bookworm as builder

RUN apt-get update \
    && apt-get install -y \
    gcc-riscv64-linux-gnu \
    gcc-aarch64-linux-gnu \
    # Perform cleanup
    && apt-get autoclean \
    && apt-get autoremove \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add riscv64gc-unknown-linux-musl && \
    rustup target add aarch64-unknown-linux-gnu

RUN cargo install --force cbindgen

COPY . .

RUN sh build-lib