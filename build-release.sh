#!/bin/bash

echo "arch: ${ARCH} for version ${VERSION} with RUSTFLAGS=${RUSTFLAGS}"
cross build -r --target "${ARCH}"
export DST="${VERSION}-${ARCH}"
mkdir -p tmp/"${DST}"/
\cp target/"${ARCH}"/release/rscan tmp/"${DST}"/
cd tmp && zip -r "${DST}".zip "rscan-${DST}"
