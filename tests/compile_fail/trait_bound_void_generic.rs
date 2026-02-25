// Non-inhabited ZST: empty enum
// Cannot construct Void, so force monomorphization via function pointer.
fn main() {
    let _: fn(foo::Void) -> foo::Void = foo::trait_bound::non_zst_only;
}
