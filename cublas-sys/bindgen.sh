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
  /opt/cuda/include/cublas.h \
  -- -I/opt/cuda/include \
  > src/cublas.rs
