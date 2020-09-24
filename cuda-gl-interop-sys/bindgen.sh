#!/bin/bash
set -exu

bindgen \
  --blacklist-type="^.*" \
  --whitelist-function="^cuda.*GL.*" \
  --default-enum-style=rust \
  --no-doc-comments \
  --with-derive-default \
  --with-derive-eq \
  --with-derive-hash \
  --with-derive-ord \
  /opt/cuda/include/cuda_gl_interop.h \
  > src/cuda_gl_interop.rs
