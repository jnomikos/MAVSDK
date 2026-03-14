use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = manifest_dir.parent().unwrap().parent().unwrap();
    let cpp_source_dir = repo_root.join("cpp");
    let c_source_dir = repo_root.join("c");

    let profile = env::var("PROFILE").unwrap();
    let cmake_build_type = if profile == "debug" { "Debug" } else { "Release" };

    let cpp_dst = cmake::Config::new(&cpp_source_dir)
        .define("CMAKE_BUILD_TYPE", cmake_build_type)
        .build();

    let prefix_path = format!(
        "{};{}/build/third_party/install", 
        cpp_dst.display(), 
        cpp_dst.display()
    );

    let c_dst = cmake::Config::new(&c_source_dir)
        .define("CMAKE_PREFIX_PATH", &prefix_path)
        .define("BUILD_SHARED_LIBS", "ON") 
        .build();

    println!("cargo:rustc-link-search=native={}/lib", c_dst.display());
    println!("cargo:rustc-link-lib=dylib=mavsdk_c");
    
    println!("cargo:rerun-if-changed={}", cpp_source_dir.display());
    println!("cargo:rerun-if-changed={}", c_source_dir.display());
}