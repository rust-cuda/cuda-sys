use env_logger;
use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "cuda-bindgen",
    about = "Generate CUDA binding to Rust",
    raw(setting = "structopt::clap::AppSettings::ColoredHelp")
)]
struct CudaBindgen {
    /// Output directory
    #[structopt(name = "output")]
    output: PathBuf,

    /// Path where the CUDA is Installed
    #[structopt(name = "cuda_path")]
    cuda_path: PathBuf,

    /// Build CUDA Driver API
    #[structopt(long = "driver")]
    driver: bool,

    /// Build CUDA Runtime API
    #[structopt(long = "runtime")]
    runtime: bool,

    /// Build cuBLAS API
    #[structopt(long = "cublas")]
    cublas: bool,

    /// Build all APIs
    #[structopt(long = "all", short = "a")]
    all: bool,
}

fn main() {
    env_logger::init();
    let opt = CudaBindgen::from_args();

    // check CUDA
    if !opt.cuda_path.is_dir() {
        panic!("CUDA does not exist at {:?}", opt.cuda_path);
    }
    let cuda_inc_path = opt.cuda_path.join("include");

    // ready output directory
    let out_path = opt.output;
    fs::create_dir_all(&out_path).expect("Unable to create output directory");

    if opt.driver || opt.all {
        bindgen::builder()
            .header(format!("{}/cuda.h", cuda_inc_path.display()))
            .clang_arg(format!("-I{}", cuda_inc_path.display()))
            .whitelist_recursively(false)
            .whitelist_type("^CU.*")
            .whitelist_type("^cuuint(32|64)_t")
            .whitelist_type("^cudaError_enum")
            .whitelist_type("^cudaMem.*")
            .whitelist_var("^CU.*")
            .whitelist_function("^CU.*")
            .whitelist_function("^cu.*")
            .default_enum_style(bindgen::EnumVariation::Rust)
            .generate()
            .expect("Unable to generate CUDA bindings")
            .write_to_file(out_path.join("cuda.rs"))
            .expect("Unable to write CUDA bindings");

        bindgen::builder()
            .header(format!("{}/cuComplex.h", cuda_inc_path.display()))
            .clang_arg(format!("-I{}", cuda_inc_path.display()))
            .whitelist_recursively(false)
            .whitelist_type("^cu.*Complex$")
            .default_enum_style(bindgen::EnumVariation::Rust)
            .generate()
            .expect("Unable to generate CUComplex bindings")
            .write_to_file(out_path.join("cucomplex.rs"))
            .expect("Unable to write CUComplex bindings");

        bindgen::builder()
            .header(format!("{}/library_types.h", cuda_inc_path.display()))
            .clang_arg(format!("-I{}", cuda_inc_path.display()))
            .whitelist_recursively(false)
            .whitelist_type("^cuda.*")
            .whitelist_type("^libraryPropertyType.*")
            .default_enum_style(bindgen::EnumVariation::Rust)
            .generate()
            .expect("Unable to generate library types bindings")
            .write_to_file(out_path.join("library_types.rs"))
            .expect("Unable to write library types bindings");

        bindgen::builder()
            .header(format!("{}/vector_types.h", cuda_inc_path.display()))
            .clang_arg(format!("-I{}", cuda_inc_path.display()))
            .whitelist_type("^u?char[0-9]$")
            .whitelist_type("^dim[0-9]$")
            .whitelist_type("^double[0-9]$")
            .whitelist_type("^float[0-9]$")
            .whitelist_type("^u?int[0-9]$")
            .whitelist_type("^u?long[0-9]$")
            .whitelist_type("^u?longlong[0-9]$")
            .whitelist_type("^u?short[0-9]$")
            .default_enum_style(bindgen::EnumVariation::Rust)
            .derive_copy(true)
            .generate()
            .expect("Unable to generate vector types bindings")
            .write_to_file(out_path.join("vector_types.rs"))
            .expect("Unable to write vector types bindings");
    }

    if opt.runtime || opt.all {
        bindgen::builder()
            .header(format!("{}/cuda_runtime.h", cuda_inc_path.display()))
            .clang_arg(format!("-I{}", cuda_inc_path.display()))
            .whitelist_recursively(false)
            .whitelist_type("^cuda.*")
            .whitelist_type("^surfaceReference")
            .whitelist_type("^textureReference")
            .whitelist_var("^cuda.*")
            .whitelist_function("^cuda.*")
            .default_enum_style(bindgen::EnumVariation::Rust)
            .generate()
            .expect("Unable to generate CUDA RT bindings")
            .write_to_file(out_path.join("cudart.rs"))
            .expect("Unable to write CUDA RT bindings");
    }

    if opt.cublas || opt.all {
        bindgen::builder()
            .header(format!("{}/cublas.h", cuda_inc_path.display()))
            .clang_arg(format!("-I{}", cuda_inc_path.display()))
            .whitelist_recursively(false)
            .whitelist_type("^cublas.*")
            .whitelist_var("^cublas.*")
            .whitelist_function("^cublas.*")
            .default_enum_style(bindgen::EnumVariation::Rust)
            .generate()
            .expect("Unable to generate CUBLAS bindings")
            .write_to_file(out_path.join("cublas.rs"))
            .expect("Unable to write CUBLAS bindings");
    }
}
