#!/bin/bash

cargo fmt --manifest-path=enfaria-common/Cargo.toml
cargo fmt --manifest-path=enfaria-web/Cargo.toml
cargo fmt --manifest-path=enfaria-server/Cargo.toml

cargo clippy --manifest-path=enfaria-web/Cargo.toml
cargo clippy --manifest-path=enfaria-server/Cargo.toml

read -p 'Press anything to close this.'
