# Humanify Duration

[<img alt="github" src="https://img.shields.io/badge/github-mrdesjardins/pretty_duration-8dagcb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/MrDesjardins/pretty-duration)
[<img alt="crates.io" src="https://img.shields.io/crates/v/pretty_duration.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/hilbert-curve-rust)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.pretty_duration-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/pretty-duration/latest/pretty-duration)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/mrdesjardins/pretty-duration/rust.yml?style=for-the-badge" height="20">](https://github.com/mrdesjardins/pretty-duration/actions?query=branch%3Amain)
[![codecov](https://codecov.io/gh/MrDesjardins/pretty-duration/branch/main/graph/badge.svg?token=TWHYC1X1KQ)](https://codecov.io/gh/MrDesjardins/pretty-duration)

Rust library that takes a duration and returns a string that is prettier to read for a human

# Consumer of the Library

## Install

```sh
cargo add pretty-duration
```

## How to use?

```sh
use pretty_duration::pretty_duration;
use std::time::Duration;
let result = pretty_duration(&Duration::from_millis(1), None);
```

# As a Developer of the Library

## Tests

```sh
cargo test
```

## Tests Coverage

You must install few components before running coverage:

```sh
cargo install grcov
rustup component add llvm-tools-preview
```

Then, you can run:

```sh
export RUSTFLAGS="-Cinstrument-coverage"
cargo build
export LLVM_PROFILE_FILE="profile-%p-%m.profraw"
cargo test
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
```

Further explanation in the [Mozilla grcov website](https://github.com/mozilla/grcov)

## Documentation

```sh
cargo doc --open
```

## Publishing

```sh
cargo login
cargo publish --dry-run
cargo publish
```