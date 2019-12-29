// pub mod cublas;
// pub mod cucomplex;
pub mod cuda;
pub mod cuda_runtime;
pub mod library_types;
pub mod vector_types;

#[test]
fn cuda_version() {
    let mut d_ver = 0;
    unsafe {
        cuda::cuDriverGetVersion(&mut d_ver as *mut i32);
    }
    println!("driver version = {}", d_ver);
}
