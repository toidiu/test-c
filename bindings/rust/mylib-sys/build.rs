fn main() {
    // Retrieve the location of `Cargo.toml`.
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    // Look for native libraries one directory higher up.
    println!(
        "cargo:rustc-link-search=native={}",
        // std::path::absolute("/home/toidiu/projects/test-c/build/lib").unwrap()
        std::path::Path::new(&dir)
            .join("..")
            .join("..")
            .join("..")
            .join("build")
            .join("lib")
            .display()
    );

    println!("cargo:rustc-link-lib=mylib");
}
