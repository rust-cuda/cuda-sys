#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

extern crate cuda_driver_sys as driver;

use driver::*;
type CUexternalMemory_st = CUexternalMemory;
type CUexternalSemaphore_st = CUexternalSemaphore;
include!("cuda_runtime.rs");
