use foo::wrapper::{self, NonZstVal};
use std::borrow::{Borrow, BorrowMut};
use std::collections::{BTreeSet, HashMap};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::mem::{align_of, size_of};
use std::rc::Rc;
use std::sync::Arc;

// ── helpers ──────────────────────────────────────────────────────────

fn hash_of<T: Hash>(val: &T) -> u64 {
    let mut h = DefaultHasher::new();
    val.hash(&mut h);
    h.finish()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Shape {
    Circle(u32),
    Rect(u32, u32),
}

// ── Layout guarantees (repr(transparent)) ───────────────────────────

macro_rules! assert_layout {
    ($ty:ty) => {
        assert_eq!(size_of::<NonZstVal<$ty>>(), size_of::<$ty>());
        assert_eq!(align_of::<NonZstVal<$ty>>(), align_of::<$ty>());
    };
}

#[test]
fn layout_u8() {
    assert_layout!(u8);
}

#[test]
fn layout_u32() {
    assert_layout!(u32);
}

#[test]
fn layout_u64() {
    assert_layout!(u64);
}

#[test]
fn layout_string() {
    assert_layout!(String);
}

#[test]
fn layout_vec_u8() {
    assert_layout!(Vec<u8>);
}

#[test]
fn layout_array_1024() {
    assert_layout!([u8; 1024]);
}

#[test]
fn layout_box_u32() {
    assert_layout!(Box<u32>);
}

#[test]
fn layout_arc_u32() {
    assert_layout!(Arc<u32>);
}

#[test]
fn layout_point_struct() {
    assert_layout!(Point);
}

#[test]
fn layout_shape_enum() {
    assert_layout!(Shape);
}

// ── Roundtrip correctness ───────────────────────────────────────────

#[test]
fn roundtrip_u32() {
    assert_eq!(NonZstVal::new(42u32).into_inner(), 42u32);
}

#[test]
fn roundtrip_f64() {
    assert_eq!(NonZstVal::new(3.14f64).into_inner(), 3.14f64);
}

#[test]
fn roundtrip_bool() {
    assert_eq!(NonZstVal::new(true).into_inner(), true);
}

#[test]
fn roundtrip_char() {
    assert_eq!(NonZstVal::new('λ').into_inner(), 'λ');
}

#[test]
fn roundtrip_string() {
    let s = String::from("hello");
    assert_eq!(NonZstVal::new(s).into_inner(), "hello");
}

#[test]
fn roundtrip_vec() {
    let v = vec![1, 2, 3];
    assert_eq!(NonZstVal::new(v).into_inner(), vec![1, 2, 3]);
}

#[test]
fn roundtrip_box() {
    let b = Box::new(99u32);
    assert_eq!(*NonZstVal::new(b).into_inner(), 99u32);
}

#[test]
fn roundtrip_from_into() {
    let w: NonZstVal<i32> = 7i32.into();
    let v: i32 = w.into_inner();
    assert_eq!(v, 7);
}

#[test]
fn roundtrip_from_trait() {
    let w = NonZstVal::from(String::from("via From"));
    assert_eq!(w.into_inner(), "via From");
}

// ── Trait impl correctness ──────────────────────────────────────────

#[test]
fn clone_equals_original() {
    let w = NonZstVal::new(String::from("clone me"));
    let c = w.clone();
    assert_eq!(w, c);
}

#[test]
fn copy_for_copy_types() {
    let w = NonZstVal::new(42u32);
    let c = w; // Copy
    assert_eq!(w, c); // both still usable
}

#[test]
fn debug_contains_inner() {
    let w = NonZstVal::new(42u32);
    let dbg = format!("{:?}", w);
    assert!(dbg.contains("42"), "Debug output was: {dbg}");
}

#[test]
fn debug_string_contains_inner() {
    let w = NonZstVal::new(String::from("hi"));
    let dbg = format!("{:?}", w);
    assert!(dbg.contains("hi"), "Debug output was: {dbg}");
}

#[test]
fn display_matches_inner() {
    let w = NonZstVal::new(42u32);
    assert_eq!(format!("{}", w), format!("{}", 42u32));
}

#[test]
fn display_string_matches_inner() {
    let s = String::from("display me");
    let w = NonZstVal::new(s.clone());
    assert_eq!(format!("{}", w), format!("{}", s));
}

#[test]
fn partial_eq_equal() {
    assert_eq!(NonZstVal::new(10u32), NonZstVal::new(10u32));
}

#[test]
fn partial_eq_not_equal() {
    assert_ne!(NonZstVal::new(10u32), NonZstVal::new(20u32));
}

#[test]
fn ord_matches_inner() {
    let a = NonZstVal::new(1u32);
    let b = NonZstVal::new(2u32);
    assert!(a < b);
    assert!(b > a);
    assert_eq!(a.cmp(&b), 1u32.cmp(&2u32));
}

#[test]
fn partial_ord_matches_inner() {
    let a = NonZstVal::new(1.0f64);
    let b = NonZstVal::new(2.0f64);
    assert!(a < b);
    assert_eq!(a.partial_cmp(&b), 1.0f64.partial_cmp(&2.0));
}

#[test]
fn hash_matches_inner() {
    let inner = String::from("hash me");
    let wrapped = NonZstVal::new(inner.clone());
    assert_eq!(hash_of(&wrapped), hash_of(&inner));
}

#[test]
fn hash_u32_matches_inner() {
    let v = 42u32;
    assert_eq!(hash_of(&NonZstVal::new(v)), hash_of(&v));
}

#[test]
fn default_matches_inner_default() {
    assert_eq!(NonZstVal::<u32>::default(), NonZstVal::new(0u32));
    assert_eq!(NonZstVal::<String>::default(), NonZstVal::new(String::new()));
    assert_eq!(NonZstVal::<bool>::default(), NonZstVal::new(false));
}

// ── Deref / DerefMut ────────────────────────────────────────────────

#[test]
fn deref_method_resolution() {
    let w = NonZstVal::new(String::from("hello"));
    assert_eq!(w.len(), 5); // .len() resolves through Deref
}

#[test]
fn deref_vec_push() {
    let mut w = NonZstVal::new(vec![1, 2, 3]);
    w.push(4); // .push() resolves through DerefMut
    assert_eq!(&*w, &[1, 2, 3, 4]);
}

#[test]
fn deref_mut_string_mutation() {
    let mut w = NonZstVal::new(String::from("hello"));
    w.push_str(" world");
    assert_eq!(&*w, "hello world");
}

#[test]
fn deref_star_operator() {
    let w = NonZstVal::new(42u32);
    assert_eq!(*w + 1, 43);
}

#[test]
fn borrow_mut_mutation() {
    let mut w = NonZstVal::new(vec![1, 2]);
    let inner: &mut Vec<i32> = w.borrow_mut();
    inner.push(3);
    assert_eq!(&*w, &[1, 2, 3]);
}

// ── AsRef / AsMut / Borrow interop ──────────────────────────────────

fn takes_as_ref<T: AsRef<Vec<u8>>>(v: &T) -> usize {
    v.as_ref().len()
}

#[test]
fn as_ref_generic_bound() {
    let w = NonZstVal::new(vec![1u8, 2, 3]);
    // NonZstVal<Vec<u8>> satisfies AsRef<Vec<u8>>, usable in generic contexts.
    assert_eq!(takes_as_ref(&w), 3);
}

#[test]
fn as_ref_direct() {
    let w = NonZstVal::new(42u32);
    let r: &u32 = w.as_ref();
    assert_eq!(*r, 42);
}

#[test]
fn as_mut_direct() {
    let mut w = NonZstVal::new(42u32);
    let r: &mut u32 = w.as_mut();
    *r = 99;
    assert_eq!(*w, 99);
}

#[test]
fn borrow_hashmap_lookup() {
    // NonZstVal<String> implements Borrow<String>, so we can look up by &String.
    let mut map = HashMap::new();
    map.insert(NonZstVal::new(String::from("key")), 42);

    let lookup = String::from("key");
    assert_eq!(map.get(&lookup), Some(&42));
}

#[test]
fn borrow_returns_inner_ref() {
    let w = NonZstVal::new(100i32);
    let b: &i32 = w.borrow();
    assert_eq!(*b, 100);
}

// ── Collection usage ────────────────────────────────────────────────

#[test]
fn vec_sorting_matches_inner() {
    let mut raw = vec![3i32, 1, 4, 1, 5, 9, 2, 6];
    let mut wrapped: Vec<NonZstVal<i32>> = raw.iter().copied().map(NonZstVal::new).collect();

    raw.sort();
    wrapped.sort();

    let unwrapped: Vec<i32> = wrapped.into_iter().map(NonZstVal::into_inner).collect();
    assert_eq!(raw, unwrapped);
}

#[test]
fn hashmap_with_nonzstval_key() {
    let mut map = HashMap::new();
    map.insert(NonZstVal::new(1u32), "one");
    map.insert(NonZstVal::new(2u32), "two");
    assert_eq!(map.get(&NonZstVal::new(1u32)), Some(&"one"));
    assert_eq!(map.get(&NonZstVal::new(2u32)), Some(&"two"));
    assert_eq!(map.get(&NonZstVal::new(3u32)), None);
}

#[test]
fn btreeset_with_nonzstval() {
    let mut set = BTreeSet::new();
    set.insert(NonZstVal::new(3));
    set.insert(NonZstVal::new(1));
    set.insert(NonZstVal::new(2));
    set.insert(NonZstVal::new(1)); // duplicate

    let vals: Vec<i32> = set.into_iter().map(NonZstVal::into_inner).collect();
    assert_eq!(vals, vec![1, 2, 3]);
}

// ── Drop correctness ────────────────────────────────────────────────

#[test]
fn drop_decrements_rc_refcount() {
    let rc = Rc::new(42u32);
    assert_eq!(Rc::strong_count(&rc), 1);

    let w = NonZstVal::new(rc.clone());
    assert_eq!(Rc::strong_count(&rc), 2);

    drop(w);
    assert_eq!(Rc::strong_count(&rc), 1);
}

#[test]
fn drop_decrements_arc_refcount() {
    let arc = Arc::new(99u32);
    assert_eq!(Arc::strong_count(&arc), 1);

    let w = NonZstVal::new(arc.clone());
    assert_eq!(Arc::strong_count(&arc), 2);

    drop(w);
    assert_eq!(Arc::strong_count(&arc), 1);
}

// ── Auto-trait forwarding ───────────────────────────────────────────

// Static assertions: NonZstVal<T> is Send/Sync when T is.
const _: fn() = || {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<NonZstVal<u32>>();
    assert_sync::<NonZstVal<u32>>();
    assert_send::<NonZstVal<String>>();
    assert_sync::<NonZstVal<String>>();
    assert_send::<NonZstVal<Arc<u32>>>();
    assert_sync::<NonZstVal<Arc<u32>>>();
};

// ── Various non-ZST types ───────────────────────────────────────────

#[test]
fn primitives_u8() {
    assert_eq!(NonZstVal::new(255u8).into_inner(), 255u8);
}

#[test]
fn primitives_i32() {
    assert_eq!(NonZstVal::new(-1i32).into_inner(), -1i32);
}

#[test]
fn primitives_u64() {
    assert_eq!(NonZstVal::new(u64::MAX).into_inner(), u64::MAX);
}

#[test]
fn primitives_f64() {
    let w = NonZstVal::new(std::f64::consts::PI);
    assert!((w.into_inner() - std::f64::consts::PI).abs() < f64::EPSILON);
}

#[test]
fn primitives_bool() {
    assert_eq!(NonZstVal::new(true).into_inner(), true);
    assert_eq!(NonZstVal::new(false).into_inner(), false);
}

#[test]
fn primitives_char() {
    assert_eq!(NonZstVal::new('🦀').into_inner(), '🦀');
}

#[test]
fn reference_str() {
    let w = NonZstVal::new("hello world");
    assert_eq!(w.into_inner(), "hello world");
}

#[test]
fn reference_slice() {
    let data: &[u8] = &[1, 2, 3];
    let w = NonZstVal::new(data);
    assert_eq!(w.into_inner(), &[1u8, 2, 3] as &[u8]);
}

#[test]
fn smart_pointer_box() {
    let w = NonZstVal::new(Box::new(42u32));
    assert_eq!(**w, 42u32);
}

#[test]
fn smart_pointer_arc() {
    let w = NonZstVal::new(Arc::new(42u32));
    assert_eq!(**w, 42u32);
}

#[test]
fn smart_pointer_rc() {
    let w = NonZstVal::new(Rc::new(42u32));
    assert_eq!(**w, 42u32);
}

#[test]
fn function_pointer() {
    fn answer() -> u32 {
        42
    }
    let fp: fn() -> u32 = answer;
    let w = NonZstVal::new(fp);
    assert_eq!(w.into_inner()(), 42);
}

#[test]
fn tuple_pair() {
    let w = NonZstVal::new((10u32, 20u32));
    assert_eq!(w.into_inner(), (10, 20));
}

#[test]
fn nested_struct() {
    let p = Point { x: 1, y: 2 };
    let w = NonZstVal::new(p.clone());
    assert_eq!(w.into_inner(), p);
}

#[test]
fn enum_with_data() {
    let s = Shape::Rect(3, 4);
    let w = NonZstVal::new(s.clone());
    assert_eq!(w.into_inner(), s);
}

#[test]
fn enum_ordering() {
    let a = NonZstVal::new(Shape::Circle(1));
    let b = NonZstVal::new(Shape::Rect(1, 1));
    // Variant order: Circle < Rect
    assert!(a < b);
}

// ── non_zst_only with impl Into ─────────────────────────────────────

#[test]
fn non_zst_only_direct_value() {
    assert_eq!(wrapper::non_zst_only(42u32), 42u32);
}

#[test]
fn non_zst_only_string() {
    assert_eq!(wrapper::non_zst_only(String::from("x")), "x");
}

#[test]
fn non_zst_only_already_wrapped() {
    // NonZstVal<T> itself is non-ZST, so passing it to non_zst_only
    // wraps it in NonZstVal<NonZstVal<T>>. The outer into_inner returns
    // NonZstVal<T>.
    let w = NonZstVal::new(42u32);
    let result: NonZstVal<u32> = wrapper::non_zst_only(w);
    assert_eq!(result.into_inner(), 42u32);
}

#[test]
fn non_zst_only_vec() {
    assert_eq!(wrapper::non_zst_only(vec![1, 2, 3]), vec![1, 2, 3]);
}

#[test]
fn non_zst_only_box() {
    let b = Box::new(7u32);
    assert_eq!(*wrapper::non_zst_only(b), 7u32);
}

#[test]
fn non_zst_only_bool() {
    assert_eq!(wrapper::non_zst_only(true), true);
}

#[test]
fn non_zst_only_tuple() {
    assert_eq!(wrapper::non_zst_only((1u32, 2u32)), (1u32, 2u32));
}
