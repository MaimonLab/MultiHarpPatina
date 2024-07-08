use std::env;

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