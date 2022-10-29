# Humanify Duration

[<img alt="github" src="https://img.shields.io/badge/github-mrdesjardins/pretty_duration-8dagcb?labelColor=555555&logo=github" height="20">](https://github.com/MrDesjardins/pretty-duration)
[<img alt="crates.io" src="https://img.shields.io/crates/v/pretty_duration.svg?color=fc8d62&logo=rust" height="20">](https://crates.io/crates/pretty-duration)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.pretty_duration-66c2a5?labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/pretty-duration/latest/pretty-duration)
[![CI Build](https://github.com/MrDesjardins/pretty-duration/actions/workflows/rust.yml/badge.svg)](https://github.com/MrDesjardins/pretty-duration/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/MrDesjardins/pretty-duration/branch/main/graph/badge.svg?token=TWHYC1X1KQ)](https://codecov.io/gh/MrDesjardins/pretty-duration)

Rust library that takes a duration and returns a string that is prettier to read for a human

# Consumer of the Library

## Install

```sh
cargo add pretty-duration
```

## How to use?

### Without Configuration Option

```rust
use pretty_duration::pretty_duration;
use std::time::Duration;
let result = pretty_duration(&Duration::from_millis(31556956789), None);
// result: 1 year 11 months 109 days 5 hours 49 minutes 16 seconds 789 milliseconds
```

### With Configuration Option - Language

```rust
use pretty_duration::pretty_duration;
use std::time::Duration;
let result = pretty_duration(
    &Duration::from_millis(31556956789),
    Some(PrettyDurationOptions {
        output_format: Some(PrettyDurationOutputFormat::Expanded),
        singular_labels: Some(PrettyDurationLabels {
            year: "année",
            month: "mois", // Not the `s` here in singular form
            day: "jour",
            hour: "heure",
            minute: "minute",
            second: "seconde",
            millisecond: "milliseconde",
        }),
        plural_labels: Some(PrettyDurationLabels {
            year: "années",
            month: "mois",
            day: "jours",
            hour: "heures",
            minute: "minutes",
            second: "secondes",
            millisecond: "millisecondes",
        }),
    }),
);
// result: "1 année 11 mois 109 jours 5 heures 49 minutes 16 secondes 789 millisecondes
```

### With Configuration Option - Compact
```rust
let result = pretty_duration(
    &Duration::from_millis(31556956789),
    Some(PrettyDurationOptions {
        output_format: Some(PrettyDurationOutputFormat::Compact),
        singular_labels: None,
        plural_labels: None,
    }),
);
//  result: "1y 11mon 109d 5h 49m 16s 789ms");
```
# As a Developer of the Library

## What to Install?

You need to install the right toolchain:

```sh
rustup toolchain install stable
rustup default stable
```

To perform test coverage you need to install

```sh
cargo install grcov
rustup component add llvm-tools-preview
```

To generate benchmark plots you need to install GnuPlot

```sh
sudo apt update
sudo apt install gnuplot

# To confirm that it is properly installed:
which gnuplot
```

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
./coverage.sh
```

Further explanation in the [Mozilla grcov website](https://github.com/mozilla/grcov)

## Documentation

```sh
cargo doc --open
```

# Benchmark

```sh
cargo bench
```

## Publishing

```sh
cargo login
cargo publish --dry-run
cargo publish
```
