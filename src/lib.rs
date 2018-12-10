#[macro_use] extern crate static_assertions;

pub mod cublas;
pub mod cuda;
pub mod cudart;
#[cfg(feature = "gte_cuda_8_0")]
pub mod cuda_fp16;
pub mod vector_types;

#[cfg(feature = "cuda_8_0")]
const_assert_eq!(cuda_8_0_api_version;  cuda::__CUDA_API_VERSION, 8000);
#[cfg(feature = "cuda_8_0")]
const_assert_eq!(cuda_8_0_version;      cuda::CUDA_VERSION,       8000);

#[test]
fn cuda_version() {
    let mut d_ver = 0;
    unsafe {
        cuda::cuDriverGetVersion(&mut d_ver as *mut i32);
    }
    println!("driver version = {}", d_ver);
}

mod cuda_tests;
mod cudart_tests;
