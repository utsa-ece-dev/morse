FROM rust:slim-bookworm

RUN apt-get update \
    && apt-get install -y \
    gcc-aarch64-linux-gnu \
    xz-utils \
    # Perform cleanup
    && apt-get autoclean \
    && apt-get autoremove \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add riscv64gc-unknown-linux-musl && \
    rustup target add aarch64-unknown-linux-gnu

RUN cargo install --force cbindgen

COPY x-tools.tar.xz ./

RUN tar xvf x-tools.tar.xz \
    && rm x-tools.tar.xz \
    && cp -r x-tools/riscv64-unknown-linux-musl . \
    && rm -rf x-tools

ENV PATH=/riscv64-unknown-linux-musl/bin:$PATH

COPY build-lib Cargo.lock Cargo.toml cbindgen.toml app/
COPY src app/src
COPY .cargo/config.toml /app/.cargo/config.toml

WORKDIR /app

RUN sh build-lib
