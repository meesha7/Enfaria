#!/bin/bash

cargo build --release --manifest-path=../enfaria-common/Cargo.toml --out-dir=../enfaria-game/src/native -Z unstable-options

read -p 'Press Enter to close this.'