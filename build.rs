use std::env;
use std::path::PathBuf;

fn main() {
    let lib = pkg_config::Config::new()
        .atleast_version("1.16.2")
        .probe("chafa")
        .expect("pkg-config: Lib `chafa` not found.");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(
            lib.include_paths
                .iter()
                .map(|p| format!("-I{}", p.display())),
        )
        .generate()
        .expect("bindgen: Failed to generate bindings.");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("bindgen: Failed to write bindings.");
}
