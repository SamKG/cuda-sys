use std::env;

use std::path::{Path, PathBuf};

pub fn include_cuda() {
    let paths = find_cuda_lib_dirs();
    if paths.is_empty() {
        panic!("Could not find a cuda installation");
    }
    for path in paths {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    println!("cargo:rustc-link-lib=dylib=cuda");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CUDA_LIBRARY_PATH");
    println!("cargo:rerun-if-env-changed=CUDA_ROOT");
    println!("cargo:rerun-if-env-changed=CUDA_PATH");
    println!("cargo:rerun-if-env-changed=CUDA_TOOLKIT_ROOT_DIR");

    let cuda_root = find_cuda_root().expect("Could not find a cuda installation");

    println!("cargo:include={}", cuda_root.join("include").display());
}

// Returns true if the given path is a valid cuda installation
fn is_cuda_root_path<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().join("include").join("cuda.h").is_file()
}

pub fn find_cuda_root() -> Option<PathBuf> {
    // search through the common environment variables first
    for path in ["CUDA_PATH", "CUDA_ROOT", "CUDA_TOOLKIT_ROOT_DIR"]
        .iter()
        .filter_map(|name| std::env::var(*name).ok())
    {
        if is_cuda_root_path(&path) {
            return Some(path.into());
        }
    }

    // If it wasn't specified by env var, try the default installation paths
    let default_paths = ["/usr/lib/cuda", "/usr/local/cuda", "/opt/cuda"];

    for path in default_paths {
        if is_cuda_root_path(path) {
            return Some(path.into());
        }
    }

    None
}

pub fn read_env() -> Vec<PathBuf> {
    if let Ok(path) = env::var("CUDA_LIBRARY_PATH") {
        // The location of the libcuda, libcudart, and libcublas can be hardcoded with the
        // CUDA_LIBRARY_PATH environment variable.
        let split_char = ":";
        path.split(split_char).map(PathBuf::from).collect()
    } else {
        vec![]
    }
}

pub fn find_cuda_lib_dirs() -> Vec<PathBuf> {
    let mut candidates = read_env();
    candidates.push(PathBuf::from("/opt/cuda"));
    candidates.push(PathBuf::from("/usr/local/cuda"));
    for e in glob::glob("/usr/local/cuda-*").unwrap().flatten() {
        candidates.push(e)
    }
    candidates.push(PathBuf::from("/usr/lib/cuda"));

    let mut valid_paths = vec![];
    for base in &candidates {
        let lib = PathBuf::from(base).join("lib64");
        if lib.is_dir() {
            valid_paths.push(lib.clone());
            valid_paths.push(lib.join("stubs"));
        }
        let base = base.join("targets/x86_64-linux");
        let header = base.join("include/cuda.h");
        if header.is_file() {
            valid_paths.push(base.join("lib"));
            valid_paths.push(base.join("lib/stubs"));
            println!("cargo:include={}", base.join("include").display());
            continue;
        }
    }
    valid_paths
}

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    include_cuda();

    let cuda_root = find_cuda_root().expect("Could not find a cuda installation");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("cuda-driver.h")
        .clang_arg(format!("-I{}", cuda_root.join("include").display()))
        .wrap_static_fns(true)
        .clang_macro_fallback()
        .derive_default(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("cuda-driver.rs"))
        .expect("Couldn't write bindings!");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("cuda-runtime.h")
        .clang_arg(format!("-I{}", cuda_root.join("include").display()))
        .wrap_static_fns(true)
        .clang_macro_fallback()
        .derive_default(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("cuda-runtime.rs"))
        .expect("Couldn't write bindings!");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("compute-sanitizer.h")
        .clang_arg(format!("-I{}", cuda_root.join("include").display()))
        .clang_arg(format!("-I{}", cuda_root.join("compute-sanitizer").join("include").display()))
        .clang_arg(format!("-L{}", cuda_root.join("compute-sanitizer").display()))
        .wrap_static_fns(true)
        .clang_macro_fallback()
        .derive_default(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("compute-sanitizer.rs"))
        .expect("Couldn't write bindings!");
}
