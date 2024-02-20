#!/bin/bash
set -ex

echo "arch: [${ARCH}] version: [${VERSION}] RUSTFLAGS: [${RUSTFLAGS}]"

cross build -r --target "${ARCH}"

export DST="rscan-${VERSION}-${ARCH}"
mkdir -p tmp/"${DST}"/
\cp target/"${ARCH}"/release/rscan tmp/"${DST}"/
cd tmp && zip -r "${DST}".zip "${DST}"
