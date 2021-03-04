#!/bin/bash
set -exu

bindgen \
  --whitelist-type="^cuda.*" \
  --whitelist-type="^surfaceReference" \
  --whitelist-type="^textureReference" \
  --whitelist-var="^cuda.*" \
  --whitelist-function="^cuda.*" \
  --default-enum-style=rust \
  --no-doc-comments \
  --with-derive-default \
  --with-derive-eq \
  --with-derive-hash \
  --with-derive-ord \
  --size_t-is-usize \
  "$CUDA_PATH/include/cuda_runtime.h" \
  > src/cuda_runtime.rs
