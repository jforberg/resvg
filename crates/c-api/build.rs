fn main() {
    println!("cargo:rustc-link-arg=-Wl,-soname,libresvg.so.0");
}
