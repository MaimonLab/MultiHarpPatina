use std::env;

#[cfg(all(feature = "nolib", feature = "MHLib"))]
compile_error!("features `nolib` and `MHLib` are mutually \
exclusive. If you want to use the `nolib` feature, you must disable \
default features `--no-default-features`.");

#[cfg(feature = "nolib")]
fn main() {}

#[cfg(feature = "MHLib")]
fn main() {
    let target = env::var("TARGET").unwrap();

    if target.contains("windows") {
        println!("cargo:rustc-link-lib=mhlib64");

        println!("cargo:rustc-link-search=native=c:\\Program Files\\PicoQuant\\MultiHarp-MHLibv30");
    }
    else {
        println!("cargo:rustc-link-lib=mhlib64");

        println!("cargo:rustc-link-search=native=/usr/local/lib");
    }
}