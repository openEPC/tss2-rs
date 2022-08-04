# tss2-ts

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![crates.io](https://img.shields.io/crates/v/tss2.svg)](https://crates.io/crates/tss2-rs)

Implements raw Rust wrapper around tpm2-tss libraries.

### Usage

Install tpm2-tss libs to the system.

Add this to your `Cargo.toml`:

```toml
[dependencies]
tss2-rs = "0.1"
```

Import necessary types and functions and use as you would do with tpm2-tss.
Do not forget to free memory allocated inside C library.
