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
    #[structopt(name = "output", help = "Output Directory")]
    output: PathBuf,
    #[structopt(name = "cuda_path", help = "Path where the CUDA is Installed")]
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
}

fn main() {
    env_logger::init();
    let opt = CudaBindgen::from_args();

    let out_path = opt.output;
    let cuda_inc_path = opt.cuda_path.join("include");

    fs::create_dir_all(&out_path).expect("Unable to create output directory");

    if opt.driver {
        bindgen::builder()
            .header(format!("{}/cuda.h", cuda_inc_path.display()))
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
            .write_to_file(out_path.join("cuda_bindings.rs"))
            .expect("Unable to write CUDA bindings");

        bindgen::builder()
            .header("wrapper/cucomplex.h")
            .clang_arg(format!("-I{}/include", cuda_inc_path.display()))
            .whitelist_recursively(false)
            .whitelist_type("^cu.*Complex$")
            .default_enum_style(bindgen::EnumVariation::Rust)
            .generate()
            .expect("Unable to generate CUComplex bindings")
            .write_to_file(out_path.join("cucomplex_bindings.rs"))
            .expect("Unable to write CUComplex bindings");
        bindgen::builder()
            .header("wrapper/library_types.h")
            .clang_arg(format!("-I{}/include", cuda_inc_path.display()))
            .whitelist_recursively(false)
            .whitelist_type("^cuda.*")
            .whitelist_type("^libraryPropertyType.*")
            .default_enum_style(bindgen::EnumVariation::Rust)
            .generate()
            .expect("Unable to generate library types bindings")
            .write_to_file(out_path.join("library_types_bindings.rs"))
            .expect("Unable to write library types bindings");

        bindgen::builder()
            .header("wrapper/vector_types.h")
            .clang_arg(format!("-I{}/include", cuda_inc_path.display()))
            // .whitelist_recursively(false)
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
            .write_to_file(out_path.join("vector_types_bindings.rs"))
            .expect("Unable to write vector types bindings");
    }

    if opt.runtime {
        bindgen::builder()
            .header("wrapper/cudart.h")
            .clang_arg(format!("-I{}/include", cuda_inc_path.display()))
            .whitelist_recursively(false)
            .whitelist_type("^cuda.*")
            .whitelist_type("^surfaceReference")
            .whitelist_type("^textureReference")
            .whitelist_var("^cuda.*")
            .whitelist_function("^cuda.*")
            .default_enum_style(bindgen::EnumVariation::Rust)
            .generate()
            .expect("Unable to generate CUDA RT bindings")
            .write_to_file(out_path.join("cudart_bindings.rs"))
            .expect("Unable to write CUDA RT bindings");
    }

    if opt.cublas {
        bindgen::builder()
            .header("wrapper/cublas.h")
            .clang_arg(format!("-I{}/include", cuda_inc_path.display()))
            .whitelist_recursively(false)
            .whitelist_type("^cublas.*")
            .whitelist_var("^cublas.*")
            .whitelist_function("^cublas.*")
            .default_enum_style(bindgen::EnumVariation::Rust)
            .generate()
            .expect("Unable to generate CUBLAS bindings")
            .write_to_file(out_path.join("cublas_bindings.rs"))
            .expect("Unable to write CUBLAS bindings");
    }
}
