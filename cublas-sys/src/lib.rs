#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

extern crate cuda_driver_sys as driver;
extern crate cuda_runtime_sys as runtime;

pub use driver::cudaDataType;
use driver::*;
use runtime::cudaStream_t;
include!("cublas.rs");
