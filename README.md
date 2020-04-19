# RusPiRo Error Handling traits

Providing the ``Error`` trait like ``std::error::Error`` for ``![no_std]`` embedded environments that come at least with
an allocator implementation.

[![Travis-CI Status](https://api.travis-ci.org/RusPiRo/ruspiro-error.svg?branch=master)](https://travis-ci.org/RusPiRo/ruspiro-error)
[![Latest Version](https://img.shields.io/crates/v/ruspiro-error.svg)](https://crates.io/crates/ruspiro-error)
[![Documentation](https://docs.rs/ruspiro-error/badge.svg)](https://docs.rs/ruspiro-error)
[![License](https://img.shields.io/crates/l/ruspiro-error.svg)](https://github.com/RusPiRo/ruspiro-error#license)


## Usage

To use the crate just add the following dependency to your ``Cargo.toml`` file:
```
[dependencies]
ruspiro-error = "0.1"
```

## Dependency

When using this crate to build a final binary the crate graph need to contain an allocator implementation. The one
used within the RusPiRo family of crates is [ruspiro-allocator](https://crates.io/crates/ruspiro-allocator)