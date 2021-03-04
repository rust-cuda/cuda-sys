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
  /opt/cuda/include/cuda_runtime.h \
  --size_t-is-usize \
  > src/cuda_runtime.rs
