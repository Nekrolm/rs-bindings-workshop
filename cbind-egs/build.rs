use std::path::PathBuf;

fn main() {
    // build and link against C/C++ dependency
    let dst = cmake::build("clib_dependency").join("lib");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=clib_dep_egs");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for
        .header("dep_wrapper.hpp")
        .clang_args(["-std=c++17", "-Iclib_dependency"])
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // clean up unwanted items
        .blocklist_item("__.*")
        .blocklist_item("_.*")
        .blocklist_item("INT.*")
        .blocklist_item("WINT.*")
        .blocklist_item("SIZE_MAX")
        .blocklist_item("UINT.*")
        .blocklist_type("int.*")
        .blocklist_type("uint.*")
        .blocklist_type("u_.*")
        .blocklist_type("syscall_arg_t")
        .blocklist_type("user_.*")
        .blocklist_type("register_t")
        .blocklist_item("SIG.*")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("clib_dep_bindings.rs"))
        .expect("Couldn't write bindings!");
}
