fn main() {
    // equivalent to -L call-c-from-rust/include
    println!("cargo:rustc-link-search=call-c-from-rust/include");
}
