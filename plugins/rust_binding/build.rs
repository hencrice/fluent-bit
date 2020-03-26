extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut binding_builder = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .whitelist_function("flb_config_map_set")
        .whitelist_function("flb_output_get_property")
        .whitelist_function("flb_pack_to_json_format_type")
        .whitelist_function("flb_pack_to_json_date_type")
        .whitelist_function("flb_config_map_set")
        .whitelist_function("flb_output_set_context")
        .whitelist_function("flb_output_return_non_inline")
        .whitelist_type("flb_input_instance")
        .whitelist_type("flb_filter_instance")
        .whitelist_type("flb_output_instance")
        .derive_debug(true)
        // blacklist the following 3 so that bindgen
        // does not generate them, which will 
        // conflict with my adapted version
        // in lib.rs
        .blacklist_type("flb_input_plugin")
        .blacklist_type("flb_filter_plugin")
        .blacklist_type("flb_output_plugin")
        .blacklist_type("flb_net_host")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    // these directories contain header files from all the C struct
    // plugins need to interact with. They were generated via cmake
    // and passed to Cargo when this crate is built.
    let bindgen_include_dirs = env::var_os("BINDGEN_HEADER_DIRS")
        .expect("BINDGEN_HEADER_DIRS not provided");
    let clang_args_str = bindgen_include_dirs.into_string()
        .expect("can not convert BINDGEN_HEADER_DIRS into String");
    let clang_args = clang_args_str.split_ascii_whitespace();
    for arg in clang_args {
        // https://stackoverflow.com/questions/42741815/setting-the-include-path-with-bindgen
        binding_builder = binding_builder.clang_arg(arg);
    }    

    // Finish the builder and generate the bindings.
    let binding = binding_builder.generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    binding
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}