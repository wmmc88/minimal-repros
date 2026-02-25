// Non-inhabited ZST: Infallible
// Cannot construct Infallible, so force monomorphization via function pointer.
fn main() {
    let _: fn(std::convert::Infallible) -> std::convert::Infallible = foo::inline_const::non_zst_only;
}
