# Humanify Duration
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