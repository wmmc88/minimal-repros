// Inhabited ZST: zero-length array
fn main() {
    foo::trait_bound::non_zst_only([0u8; 0]);
}
