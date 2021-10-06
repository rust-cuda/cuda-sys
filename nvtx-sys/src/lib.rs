#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![allow(deref_nullptr)] // https://github.com/rust-lang/rust-bindgen/issues/1651
include!("nv_tools_ext.rs");

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    // not a test for cublas initialization, but a test for linking cublasCreate_v2
    #[test]
    fn link_test() {
        unsafe {
            let name = CStr::from_bytes_with_nul_unchecked(b"my mark\0");
            nvtxMarkA(name.as_ptr());
        }
    }
}
