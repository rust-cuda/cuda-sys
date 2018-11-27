cuda-sys
=========

[![CircleCI](https://circleci.com/gh/rust-cuda/cuda-sys/tree/master.svg?style=shield)](https://circleci.com/gh/rust-cuda/cuda-sys/tree/master)
[![Crate](http://meritbadge.herokuapp.com/cuda-sys)](https://crates.io/crates/cuda-sys)
[![docs.rs](https://docs.rs/cuda-sys/badge.svg)](https://docs.rs/cuda-sys)

Rust binding to CUDA Driver(`libcuda.so`)/Runtime(`libcudart.so`) APIs

- This crate does not include CUDA itself. You need to install on your own.
- `$CUDA_LIBRARY_PATH` (e.g. `/opt/cuda/lib64`) will be used for `cargo:rustc-link-search`
