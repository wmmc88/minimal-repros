use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Build libfoo (production library)
    // cargo_metadata(false) is CRITICAL: prevents automatic linking so we can
    // use conditional #[cfg] attributes to control which library gets linked
    cc::Build::new()
        .file("src/libfoo.c")
        .cargo_metadata(false)
        .compile("foo");
    
    // Build libtestfoo (test library) 
    // cargo_metadata(false) is CRITICAL: prevents automatic linking so we can
    // use conditional #[cfg] attributes to control which library gets linked
    cc::Build::new()
        .file("src/libtestfoo.c")
        .cargo_metadata(false)
        .compile("testfoo");
    
    // Tell rustc where to find the libraries (but don't link them yet)
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    
    // Print build info for debugging
    println!("cargo:warning=Libraries built in: {}", out_dir.display());
}