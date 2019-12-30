cuda-sys
=========

[![Crate](http://meritbadge.herokuapp.com/cuda-sys)](https://crates.io/crates/cuda-sys)
[![docs.rs](https://docs.rs/cuda-sys/badge.svg)](https://docs.rs/cuda-sys)

Rust binding to [CUDA Driver (`libcuda.so`)](https://docs.nvidia.com/cuda/cuda-driver-api/index.html)
and [Runtime (`libcudart.so`) APIs](https://docs.nvidia.com/cuda/cuda-runtime-api/index.html)

This crate does **NOT** include CUDA itself. You need to install on your own.

CUDA Installation
------------------

- You should install CUDA from [the official installer](https://developer.nvidia.com/cuda-downloads).
- or use [`nvidia/cuda` container](https://hub.docker.com/r/nvidia/cuda/)
  - This container does not add `libcuda.so` to dynamic link path.
    You need to add it to your ld path `LD_LIBRARY_PATH=/usr/local/cuda/lib64/stubs` or using `ldconfig`.

CUDA_LIBRARY_PATH environment variable
--------------------------------------

- If you use system installer (e.g. `apt`, `yum`, `pacman`, and so on), the install path may be different.
- `$CUDA_LIBRARY_PATH` (e.g. `/opt/cuda/lib64`) environment value adds paths to the list of library searching.
  - `$CUDA_LIBRARY_PATH` is not required on Windows when CUDA is installed via [the official Windows installer](https://developer.nvidia.com/cuda-downloads).
- To verify that cuda-sys can find your CUDA installation, you can check that the `CUDA_PATH` environment variable has been set.

Q & A
------

Q. Program does not start with following error message:

```
error while loading shared libraries: libcuda.so.1: cannot open shared object file: No such file or directory
```

A. Driver API (`libcuda.so`) is sometimes installed in a separate directory. You need to find it, and add to `LD_LIBRARY_PATH`:

```
LD_LIBRARY_PATH=/usr/local/cuda/lib64/stubs
```

License
--------
Dual-licensed to be compatible with the Rust project.
Licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or the [MIT license](http://opensource.org/licenses/MIT), at your option.
This file may not be copied, modified, or distributed except according to those terms.

You should check the [End User License Agreement](https://docs.nvidia.com/cuda/eula/index.html),
which describes NVIDIA Software License Agreement and CUDA Supplement to Software License Agreement.
