#!/bin/bash

U="cosmwasm"
V="0.13.0"

M=$(uname -m)

A="linux/${M/x86_64/amd64}"
S=${M#x86_64}
S=${S:+-$S}

BN=$(basename "$(pwd)")

(
    cd ../..

    docker run --platform $A --rm -v "$(pwd)":/code \
    --mount type=volume,source="$BN_cache",target=/code/contracts/$BN/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    $U/rust-optimizer$S:$V ./contracts/$BN
)

# mv ../../artifacts/filter-aarch64.wasm ./artifacts/filter.wasm