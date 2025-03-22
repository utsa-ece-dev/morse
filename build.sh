#!/bin/bash

docker build -t morse-lib .
container_id=$(docker create morse-lib:latest)
docker cp $container_id:/app/target/riscv64gc-unknown-linux-musl/release/libmorse.so libmorse-riscv64.so
docker cp $container_id:/app/target/aarch64-unknown-linux-gnu/release/libmorse.so libmorse-aarch64.so
docker cp $container_id:/app/morse.h morse.h
docker stop $container_id
docker rm $container_id
