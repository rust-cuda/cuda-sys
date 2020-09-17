#!/bin/bash
set -exu

bindgen \
  --blacklist-type="^.*" \
  --whitelist-var="^CU.*" \
  --whitelist-function="^cu.*" \
  --default-enum-style=rust \
  --no-doc-comments \
  --with-derive-default \
  --with-derive-eq \
  --with-derive-hash \
  --with-derive-ord \
  wrapper.h -- -I/opt/cuda/include \
  > src/cuda.rs
