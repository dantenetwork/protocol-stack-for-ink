#!/bin/bash

cargo +nightly test

export PATH=~/env/rust/binaryen-version_105/bin/:$PATH

cargo +nightly contract build

cargo +nightly contract build --release

cargo +nightly test
