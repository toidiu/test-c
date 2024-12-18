fn main() {
    println!("cargo:rustc-link-lib=libmylib");

    // Retrieve the location of `Cargo.toml`.
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    // Look for native libraries one directory higher up.
    println!(
        "cargo:rustc-link-search=native={}",
        std::path::Path::new(&dir)
            .join("..")
            .join("..")
            .join("build")
            .join("lib")
            .display()
    );
}
