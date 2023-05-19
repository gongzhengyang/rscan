#!/bin/bash

echo "arch: ${ARCH} for version ${VERSION}"
cross build -r --target "${ARCH}"
export DST="${VERSION}-${ARCH}"
mkdir -p tmp/"${DST}"/
\cp target/"${ARCH}"/release/rscan tmp/"${DST}"/
cd tmp && zip -r "${DST}".zip "${DST}"
