#!/bin/bash
set -exu

bindgen \
  --whitelist-type="^CU.*" \
  --whitelist-type="^cuuint(32|64)_t" \
  --whitelist-type="^cudaError_enum" \
  --whitelist-type="^cu.*Complex$" \
  --whitelist-type="^cuda.*" \
  --whitelist-type="^surfaceReference" \
  --whitelist-type="^textureReference" \
  --whitelist-type="^libraryPropertyType.*" \
  --default-enum-style=rust \
  --no-doc-comments \
  --with-derive-default \
  --with-derive-eq \
  --with-derive-hash \
  --with-derive-ord \
  wrapper.h -- -I/opt/cuda/include \
  > src/cuda_types.rs
