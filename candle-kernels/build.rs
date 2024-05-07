use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    env::set_var("CUDA_COMPUTE_CAP", "70");
    let builder = bindgen_cuda::Builder::default();
    println!("cargo:info={builder:?}");
    let bindings = builder.build_ptx().unwrap();
    bindings.write("src/lib.rs").unwrap();
}
