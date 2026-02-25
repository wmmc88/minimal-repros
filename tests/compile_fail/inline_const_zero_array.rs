// Inhabited ZST: zero-length array
fn main() {
    foo::inline_const::non_zst_only([0u8; 0]);
}
