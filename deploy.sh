#!/bin/bash -e

TARGET=aarch64-unknown-linux-gnu
USER=pi

# Build the html files
cd client
pnpm run build

# Go back home
cd ..

# Build the server
cd server
cargo install -f cross
cross build --target $TARGET --release

# TODO: Copy the binary to the target
# TODO: Execute the binary on the target