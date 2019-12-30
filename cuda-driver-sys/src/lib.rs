#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
include!("cuda.rs");

#[cfg(test)]
mod tests {
    use super::*;

    // not a test for get driver version, but a test for linking
    #[test]
    fn link_test() {
        let mut version: i32 = 0;
        let result = unsafe { cuDriverGetVersion(&mut version as *mut i32) };
        match result {
            CUresult::CUDA_SUCCESS => {
                println!("Deriver Version = {:?}", version);
            }
            _ => {
                println!("Cannot get driver version");
            }
        }
    }
}
