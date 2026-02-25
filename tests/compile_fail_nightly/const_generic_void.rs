// Non-inhabited ZST: empty enum
// Cannot construct Void, so force monomorphization via function pointer.
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
fn main() {
    let _: fn(foo::Void) -> foo::Void = foo::const_generic::non_zst_only;
}
