#!/bin/bash
set -exu

bindgen \
  --whitelist-type="^cusolver.*" \
  --whitelist-function="^cusolver.*" \
  --default-enum-style=rust \
  --no-doc-comments \
  --with-derive-default \
  --with-derive-eq \
  --with-derive-hash \
  --with-derive-ord \
  cusolver_wrapper.h \
  -- -I/opt/cuda/include \
  > src/cusolver.rs
