#!/bin/bash

# Script from https://medium.com/swlh/compiling-rust-for-raspberry-pi-arm-922b55dbb050

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly NAME=drinkmixer-pi
readonly TARGET_HOST=pi@pi
readonly TARGET_PATH=/srv/drinkmixer
readonly TARGET_ARCH=aarch64-unknown-linux-gnu
readonly SOURCE_PATH=target/${TARGET_ARCH}/release/${NAME}

cd pi

cargo build --target ${TARGET_ARCH} --release
rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
rsync config.ron ${TARGET_HOST}:${TARGET_PATH}
ssh -t ${TARGET_HOST} sudo systemctl restart drinkmixer.service