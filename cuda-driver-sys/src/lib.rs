#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
include!("cuda.rs");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
