#!/bin/bash
set -exu

bindgen \
  --whitelist-type="^nvtx.*" \
  --whitelist-var="^nvtx.*" \
  --whitelist-function="^nvtx.*" \
  --default-enum-style=rust \
  --no-doc-comments \
  --with-derive-default \
  --with-derive-eq \
  --with-derive-hash \
  --with-derive-ord \
  --size_t-is-usize \
  "$CUDA_PATH/include/nvToolsExt.h" \
  > src/nv_tools_ext.rs
