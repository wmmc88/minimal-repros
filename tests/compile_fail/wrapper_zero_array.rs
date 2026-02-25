// Inhabited ZST: zero-length array
fn main() {
    foo::wrapper::non_zst_only([0u8; 0]);
}
