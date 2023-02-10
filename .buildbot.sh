#! /bin/sh

set -eu

export CARGO_HOME="${PWD}/.cargo"
export RUSTUP_HOME="${PWD}/.rustup"

# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh
sh rustup.sh --default-host x86_64-unknown-linux-gnu --default-toolchain stable -y --no-modify-path

export PATH="${PWD}/.cargo/bin/:$PATH"

cargo fmt --all -- --check
cargo test
cargo test --release

# deny check
which cargo-deny | cargo install cargo-deny
cargo-deny check license
