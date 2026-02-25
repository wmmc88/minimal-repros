// Inhabited ZST: unit struct
fn main() {
    foo::inline_const::non_zst_only(foo::Empty);
}
