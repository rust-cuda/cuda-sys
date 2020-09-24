#!/bin/bash
set -exu

bindgen \
  --blacklist-type="^.*" \
  --whitelist-var="^cuda.*" \
  --whitelist-function="^cuda.*" \
  --default-enum-style=rust \
  --no-doc-comments \
  --with-derive-default \
  --with-derive-eq \
  --with-derive-hash \
  --with-derive-ord \
  /opt/cuda/include/cuda_runtime.h \
  > src/cuda_runtime.rs
