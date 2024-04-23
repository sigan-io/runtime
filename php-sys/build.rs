use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=/opt/lib");

    // Tell cargo to tell rustc to link the PHP shared library
    println!("cargo:rustc-link-lib=php");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // CLang arguments including all the needed dirs.
        .clang_arg("-I/tmp/build/php")
        .clang_arg("-I/tmp/build/php/sapi")
        .clang_arg("-I/tmp/build/php/main")
        .clang_arg("-I/tmp/build/php/Zend")
        .clang_arg("-I/tmp/build/php/TSRM")
        // Allows PHP Embed SAPI functions.
        .allowlist_function("php_embed_init")
        .allowlist_function("php_embed_shutdown")
        // Allows PHP SAPI functions.
        .allowlist_function("php_request_startup")
        .allowlist_function("php_request_shutdown")
        .allowlist_function("php_execute_script")
        // Allows Zend functions.
        .allowlist_function("zend_eval_string")
        .allowlist_function("zend_string_init")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Format output.
        .formatter(bindgen::Formatter::Prettyplease)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("./src/");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
