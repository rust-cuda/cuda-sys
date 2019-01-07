#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

pub mod driver {
    include!("bindgen/cuda.rs");
    include!("bindgen/cucomplex.rs");
    include!("bindgen/vector_types.rs");
    include!("bindgen/library_types.rs");
}

pub mod runtime {
    use driver::*;
    type CUexternalMemory_st = CUexternalMemory;
    type CUexternalSemaphore_st = CUexternalSemaphore;
    include!("bindgen/cuda_runtime.rs");
}
