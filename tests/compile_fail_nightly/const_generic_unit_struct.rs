// Inhabited ZST: unit struct
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
fn main() {
    foo::const_generic::non_zst_only(foo::Empty);
}
