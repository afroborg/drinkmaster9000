#!/bin/bash -e

readonly NAME=drinkmixer-pi
readonly TARGET_HOST=pi@pi
readonly TARGET_PATH=/srv/drinkmixer
readonly TARGET_ARCH=aarch64-unknown-linux-gnu

readonly CLIENT_SOURCE_PATH=dist/
readonly SERVER_SOURCE_PATH=target/${TARGET_ARCH}/release/${NAME}


# check that the first argument is either frontend or pi, or that it is empty
if [[ $1 == "frontend" ]] || [[ -z $1 ]]; then

    # remove the static folder on the pi
    ssh -t ${TARGET_HOST} rm -rf ${TARGET_PATH}/static >> /dev/null

    cd client

    # build the frontend
    pnpm run build

    # copy the static folder to the pi
    rsync -r ${CLIENT_SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}/static
    
    cd ..
fi

if [[ $1 == "pi" ]] || [[ -z $1 ]]; then
    # Script from https://medium.com/swlh/compiling-rust-for-raspberry-pi-arm-922b55dbb050

    set -o errexit
    set -o nounset
    set -o pipefail
    set -o xtrace

    cd pi

    # build the server
    cargo build --target ${TARGET_ARCH} --release

    # copy the server to the pi
    rsync ${SERVER_SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}

    # run the rsync if a second argument is given, and it is "config"

    if [[ $# -ge 2 && "$2" == "config" ]]; then
        # copy the config to the pi
        rsync config.ron ${TARGET_HOST}:${TARGET_PATH}
    fi

    # restart the service
    ssh -t ${TARGET_HOST} sudo systemctl restart drinkmixer.service
fi