use bindgen::builder;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-flags=-L /usr/local/lib/");
    println!("cargo:rustc-link-search=/usr/local/lib/");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=Ultralight");
        println!("cargo:rustc-link-lib=dylib=WebCore");
        println!("cargo:rustc-link-lib=dylib=AppCore");
    } else {
        println!("cargo:rustc-link-lib=Ultralight");
        println!("cargo:rustc-link-lib=WebCore");
        println!("cargo:rustc-link-lib=AppCore");
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper/wrapper.h")
        .impl_debug(true)
        .impl_partialeq(true)
        .generate_comments(true)
        .generate_inline_functions(true)
        .whitelist_var("^UL.*|JS.*|ul.*|WK.*")
        .whitelist_type("^UL.*|JS.*|ul.*|WK.*")
        .whitelist_function("^UL.*|JS.*|ul.*|WK.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
