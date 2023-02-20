#!/bin/bash -e

TARGET=aarch64-unknown-linux-gnu
USER=pi

# Build the project
cargo build --release --target $TARGET