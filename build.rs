fn main() {
    // Cmake edlib and link it.
    let out_dir = cmake::Config::new("edlib")
        .cflag("-D CMAKE_BUILD_TYPE=Release")
        .build();
    println!("cargo:rustc-link-search=native={}/lib", out_dir.display());
    println!("cargo:rustc-link-lib=edlib");

    // Link the standard library.
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=c++");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }

    // Generate bindings when the header file changes.
    let impl_path = "edlib/edlib/src/edlib.cpp";
    println!("cargo:rerun-if-changed={}", impl_path);
    let header_path = "edlib/edlib/include/edlib.h";
    println!("cargo:rerun-if-changed={}", header_path);

    // Get the output directory from cargo
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindgen::Builder::default()
        .header(header_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
