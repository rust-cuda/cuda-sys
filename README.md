cuda-sys
=========

[![Crate](http://meritbadge.herokuapp.com/cuda-sys)](https://crates.io/crates/cuda-sys)
[![docs.rs](https://docs.rs/cuda-sys/badge.svg)](https://docs.rs/cuda-sys)

Rust binding to CUDA Driver(`libcuda.so`) and Runtime(`libcudart.so`) APIs

- This crate does not include CUDA itself. You need to install on your own.
- cuda-sys searches `/usr/local/cuda`, `/opt/cuda`, and directories specified in `$CUDA_LIBRARY_PATH` while compiling.
