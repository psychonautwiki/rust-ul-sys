fn main() {
    println!("cargo:rustc-flags=-L /usr/local/lib/");
    println!("cargo:rustc-link-search=/usr/local/lib/");
    println!("cargo:rustc-link-lib=Ultralight");
    println!("cargo:rustc-link-lib=WebCore");
    println!("cargo:rustc-link-lib=AppCore");
}

// bindgen --use-core --impl-debug --impl-partialeq --generate-inline-functions --dump-preprocessed-input --conservative-inline-namespaces --whitelist-function "^UL.*|JS.*|ul.*|WK.*" --whitelist-var "^UL.*|JS.*|ul.*|WK.*" --whitelist-type "^UL.*|JS.*|ul.*|WK.*"
