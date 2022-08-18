#!/bin/bash
cargo +nightly contract build
mkdir -p ./res && cp target/ink/cross_chain.contract ./res/