#!/bin/bash -e

cd client
pnpm run build

readonly TARGET_HOST=pi@pi
readonly TARGET_PATH=/srv/drinkmixer/static
readonly SOURCE_PATH=dist/

rsync -r ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}