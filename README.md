# Embedded test issue

This is a minimal project showing how to set up embedded test using the "external-executor" feature.

## Probe-rs
Running with this version:

```
probe-rs --version
probe-rs 0.27.0 (git commit: v0.27.0-47-g12169c6b)
```

Install via `cargo binstall probe-rs-tools --version 0.27`

## Running the tests

`cargo test --features defmt`

or

`cargo test --features log`
