// change log: replace cuda-config to custom config
use std::{env, path::PathBuf};

pub fn find_cuda() -> Vec<PathBuf> {
    let split_char;
    let dir_names;
    let lib_name;

    if cfg!(target_os = "windows") {
        split_char = ";";
        dir_names = vec!["", "x64", "lib\\x64"];
        lib_name = "cuda.lib";
    } else {
        split_char = ":";
        dir_names = vec!["", "stubs", "lib64/stubs", "targets/x86_64-linux", "targets/aarch64-linux"];
        lib_name = "libcuda.so";
    }

    let mut candidates: Vec<PathBuf> = env::var("CUDA_LIBRARY_PATH").unwrap_or_default()
        .split(split_char).map(|s| PathBuf::from(s)).collect();

    env::var("CUDA_PATH").unwrap_or_default()
        .split(split_char).for_each(|s| candidates.push(PathBuf::from(s)));

    let mut valid_paths = vec![];
    let mut target;

    if cfg!(not(target_os = "windows") ) {
        candidates.push(PathBuf::from("/opt/cuda"));
        candidates.push(PathBuf::from("/usr/local/cuda"));
        candidates.push(PathBuf::from("/usr/local/cuda-11.5"));
    }

    for base in &candidates {
        if base.is_dir() {
            for dir_name in &dir_names {
                target = base.join(dir_name);
                if target.is_dir() && target.join(lib_name).is_file() {
                    valid_paths.push(target);
                }
            }
        }
    }

    if valid_paths.is_empty() {
        if cfg!(not(target_os = "windows") ) { // try find the libcuda.so with gpu driver
            target = PathBuf::from("/usr/lib/aarch64-linux-gnu");
            if target.is_dir() && target.join(lib_name).is_file() {
                valid_paths.push(target);
            }
            target = PathBuf::from("/usr/lib/x86_64-linux-gnu");
            if target.is_dir() && target.join(lib_name).is_file() {
                valid_paths.push(target);
            }
        }
        if valid_paths.is_empty() {
            panic!("CUDA cannot find! You can set the environment 'CUDA_LIBRARY_PATH' or 'CUDA_PATH' to specify it");
        }
    }

    eprintln!("Found CUDA paths: {:?}", valid_paths);
    valid_paths
}


fn main() {
    for path in find_cuda() {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    println!("cargo:rustc-link-lib=dylib=cuda");
    println!("cargo:rerun-if-changed=build.rs");
}
