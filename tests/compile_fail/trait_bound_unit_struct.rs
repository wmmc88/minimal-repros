// Inhabited ZST: unit struct
fn main() {
    foo::trait_bound::non_zst_only(foo::Empty);
}
