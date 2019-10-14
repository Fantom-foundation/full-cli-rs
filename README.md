full-cli-rs
===========
[![Build Status](https://travis-ci.org/Fantom-foundation/full-cli-rs.svg?branch=master)](https://travis-ci.org/Fantom-foundation/full-cli-rs)

full-cli-rs in Rust.

## Purpose

  0. Generate keys
  1. Setup network
  2. Run serve on each node in network
  3. Run transactions against networks (tester)
  4. Collect metrics (TPS, TTF, &etc.)
  5. Parse metrics

The idea is this could run as a CI/CD task.

## RFCs

https://github.com/Fantom-foundation/fantom-rfcs

# Developer guide

Install the latest version of [Rust](https://www.rust-lang.org). We tend to use nightly versions. [CLI tool for installing Rust](https://rustup.rs).

We use [rust-clippy](https://github.com/rust-lang-nursery/rust-clippy) linters to improve code quality.

There are plenty of [IDEs](https://areweideyet.com) and other [Rust development tools to consider](https://github.com/rust-unofficial/awesome-rust#development-tools).

### Step-by-step guide
```bash
# Install Rust (nightly)
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly
# Install cargo-make (cross-platform feature-rich reimplementation of Make)
$ cargo install --force cargo-make
# Install rustfmt (Rust formatter)
$ rustup component add rustfmt
# Clone this repo
$ git clone https://github.com/Fantom-foundation/full-cli-rs && cd full-cli-rs
# Run tests
$ cargo test
# Format, build and test
$ cargo make
# Run three nodes in a test configuration in three separate consoles:
$ RUST_LOG=debug cargo run -- -c config/config.toml -p 10001 -n 11001
$ RUST_LOG=debug cargo run -- -c config/config.toml -p 10002 -n 11002
$ RUST_LOG=debug cargo run -- -c config/config.toml -p 10003 -n 11003
```
