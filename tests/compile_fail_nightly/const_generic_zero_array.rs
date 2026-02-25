// Inhabited ZST: zero-length array
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
fn main() {
    foo::const_generic::non_zst_only([0u8; 0]);
}
