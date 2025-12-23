fn main() {
    let CARGO_ENCODED_RUSTFLAGS  = std::env::var("CARGO_ENCODED_RUSTFLAGS").unwrap_or("empty".to_string());
    println!("ENCODED FLAGS:{}", CARGO_ENCODED_RUSTFLAGS);
}