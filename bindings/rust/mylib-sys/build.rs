use std::path::Path;

fn main() {
    // Retrieve the location of `Cargo.toml`.
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // Look for native libraries directory higher up.
    let lib_root = Path::new(&dir) // lib_root/bindings/rust/mylib-sys
        .join("..") // lib_root/bindings/rust
        .join("..") // lib_root/bindings
        .join("..") // lib_root
        .canonicalize()
        .unwrap();

    let lib_path = lib_root.join("build").join("lib");

    let lib_header = lib_root.join("api").join("mylib.h");
    let lib_header = lib_header.to_str().unwrap();

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=mylib");

    let bindings = bindgen::Builder::default()
        .header(lib_header)
        .generate()
        .unwrap();

    // Write the bindings to the bindings$OUT_DIR/bindings.rs file.
    let out_path = Path::new(&dir).join("src").join("ffi.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
