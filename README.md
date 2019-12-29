cuda-sys
=========

[![Crate](http://meritbadge.herokuapp.com/cuda-sys)](https://crates.io/crates/cuda-sys)
[![docs.rs](https://docs.rs/cuda-sys/badge.svg)](https://docs.rs/cuda-sys)

Rust binding to CUDA Driver(`libcuda.so`)/Runtime(`libcudart.so`) APIs

This crate does not include CUDA itself. You need to install on your own.

`$CUDA_LIBRARY_PATH` (e.g. `/opt/cuda/lib64`) will be used for
`cargo:rustc-link-search`. `$CUDA_LIBRARY_PATH` is not required on Windows when
Cuda is installed via the typical Windows installer. To verify that cuda-sys can
find your Cuda installation, you can check that the `CUDA_PATH` environment
variable has been set.

License
--------
Dual-licensed to be compatible with the Rust project.
Licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or the [MIT license](http://opensource.org/licenses/MIT), at your option.
This file may not be copied, modified, or distributed except according to those terms.
