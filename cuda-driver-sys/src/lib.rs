#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
include!("cuda.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_version() {
        let mut version: i32 = 0;
        let result = unsafe { cuDriverGetVersion(&mut version as *mut i32) };
        if result != CUresult::CUDA_SUCCESS {
            panic!("Cannot get driver version: ERROR={:?}", result);
        }
        println!("Version = {}", version);
    }
}
