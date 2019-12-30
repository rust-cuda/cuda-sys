#![allow(non_camel_case_types)]
include!("cublas.rs");

#[cfg(test)]
mod tests {
    use super::*;

    // not a test for cublas initialization, but a test for linking cublasCreate_v2
    #[test]
    fn link_test() {
        let mut handle = std::ptr::null_mut();
        let result = unsafe { cublasCreate_v2(&mut handle as *mut _) };
        match result {
            cublasStatus_t::CUBLAS_STATUS_SUCCESS => {
                println!("Succeed to init CUBLAS");
            }
            cublasStatus_t::CUBLAS_STATUS_NOT_INITIALIZED => {
                println!("GPU not found");
            }
            _ => {
                panic!("Failed to init CUBLAS");
            }
        }
    }
}
