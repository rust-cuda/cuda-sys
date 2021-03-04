#!/bin/bash
set -exu

bindgen \
  --whitelist-type="^cublas.*" \
  --whitelist-function="^cublas.*" \
  --default-enum-style=rust \
  --no-doc-comments \
  --with-derive-default \
  --with-derive-eq \
  --with-derive-hash \
  --with-derive-ord \
  --size_t-is-usize \
  "$CUDA_PATH/include/cublas.h" \
  -- -I"$CUDA_PATH/include" \
  > src/cublas.rs
