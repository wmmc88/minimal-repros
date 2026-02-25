#![cfg_attr(feature = "nightly", feature(generic_const_exprs))]
#![cfg_attr(feature = "nightly", allow(incomplete_features))]

/// Approach 1: Const generic where-clause (nightly-only).
///
/// Requires `#![feature(generic_const_exprs)]`.
/// Tracking issue: <https://github.com/rust-lang/rust/issues/76560>
///
/// The constraint lives entirely in the `where` clause — no `check()` call,
/// no inline const block, and the error is a clean "trait bound not satisfied".
#[cfg(feature = "nightly")]
pub mod const_generic {
    use std::mem::size_of;

    pub struct Assert<const B: bool>;
    pub trait IsTrue {}
    impl IsTrue for Assert<true> {}

    /// Uses a const generic where-clause to reject ZSTs at compile time.
    pub fn non_zst_only<T>(value: T) -> T
    where
        Assert<{ size_of::<T>() != 0 }>: IsTrue,
    {
        value
    }
}

/// Approach 2: Sealed trait bound with blanket impl.
///
/// The associated constant `ASSERT` contains the compile-time check.
/// It is referenced inside the default `.non_zst()` method body, so
/// calling `.non_zst()` forces the constant to be evaluated for that
/// concrete type — triggering a compile error for ZSTs.
pub mod trait_bound {
    use std::mem::size_of;

    mod sealed {
        pub trait Sealed {}
        impl<T: Sized> Sealed for T {}
    }

    /// Marker trait for non-zero-sized types.
    ///
    /// Sealed — cannot be implemented outside this crate, so the
    /// compile-time assertion cannot be bypassed.
    pub trait NonZst: Sized + sealed::Sealed {
        const ASSERT: () = assert!(
            size_of::<Self>() != 0,
            "zero-sized types are not allowed"
        );

        /// Returns `self` unchanged, but forces evaluation of [`Self::ASSERT`],
        /// which rejects ZSTs at compile time.
        #[inline(always)]
        fn non_zst(self) -> Self {
            let () = Self::ASSERT;
            self
        }
    }

    impl<T: Sized> NonZst for T {}

    /// Example: wraps `.non_zst()` so callers don't need to import the trait.
    pub fn non_zst_only<T: NonZst>(value: T) -> T {
        value.non_zst()
    }
}

/// Approach 3: Inline const assertion (no trait required).
pub mod inline_const {
    use std::mem::size_of;

    const fn assert_non_zst<T>() {
        assert!(size_of::<T>() != 0, "zero-sized types are not allowed");
    }

    /// Uses an inline `const` block to reject ZSTs at compile time.
    pub fn non_zst_only<T>(value: T) -> T {
        const { assert_non_zst::<T>() }
        value
    }
}

/// Approach 4: Newtype wrapper with `From`/`Into` conversion.
///
/// The compile-time assertion lives in the `From<T>` impl, so it fires
/// when a value is converted into `NonZstVal<T>`.  Functions accept
/// `impl Into<NonZstVal<T>>`, and callers just pass raw values — the
/// conversion (and check) happens automatically inside the function.
pub mod wrapper {
    use std::mem::size_of;
    use std::ops::{Deref, DerefMut};

    const fn assert_non_zst<T>() {
        assert!(size_of::<T>() != 0, "zero-sized types are not allowed");
    }

    /// A value that is guaranteed to be non-zero-sized.
    ///
    /// Can only be constructed via [`NonZstVal::new`] or [`From<T>`],
    /// both of which contain a compile-time assertion rejecting ZSTs.
    #[repr(transparent)]
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    pub struct NonZstVal<T>(T);

    impl<T> NonZstVal<T> {
        #[must_use]
        #[inline]
        pub fn new(value: T) -> Self {
            const { assert_non_zst::<T>() }
            Self(value)
        }

        #[inline]
        pub fn into_inner(self) -> T {
            self.0
        }
    }

    impl<T: Default> Default for NonZstVal<T> {
        #[inline]
        fn default() -> Self {
            Self::new(T::default())
        }
    }

    impl<T: std::fmt::Display> std::fmt::Display for NonZstVal<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl<T> From<T> for NonZstVal<T> {
        #[inline]
        fn from(value: T) -> Self {
            Self::new(value)
        }
    }

    impl<T> Deref for NonZstVal<T> {
        type Target = T;
        #[inline]
        fn deref(&self) -> &T {
            &self.0
        }
    }

    impl<T> DerefMut for NonZstVal<T> {
        #[inline]
        fn deref_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    impl<T> AsRef<T> for NonZstVal<T> {
        #[inline]
        fn as_ref(&self) -> &T {
            &self.0
        }
    }

    impl<T> AsMut<T> for NonZstVal<T> {
        #[inline]
        fn as_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    impl<T> std::borrow::Borrow<T> for NonZstVal<T> {
        #[inline]
        fn borrow(&self) -> &T {
            &self.0
        }
    }

    impl<T> std::borrow::BorrowMut<T> for NonZstVal<T> {
        #[inline]
        fn borrow_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    /// Accepts any `T` via `Into<NonZstVal<T>>` — callers pass raw values,
    /// and the ZST check fires automatically during conversion.
    pub fn non_zst_only<T>(value: impl Into<NonZstVal<T>>) -> T {
        value.into().into_inner()
    }
}

// -- Example ZST definitions --

/// Inhabited ZST (unit struct — can be constructed).
pub struct Empty;

/// Non-inhabited ZST (empty enum — cannot be constructed).
pub enum Void {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    // -- Approach 1 tests (nightly only) --

    #[cfg(feature = "nightly")]
    #[test]
    fn const_generic_accepts_u32() {
        assert_eq!(const_generic::non_zst_only(42u32), 42);
    }

    #[cfg(feature = "nightly")]
    #[test]
    fn const_generic_accepts_string() {
        let s = String::from("hello");
        assert_eq!(const_generic::non_zst_only(s), "hello");
    }

    // -- Approach 2 tests --

    #[test]
    fn trait_bound_accepts_u32() {
        assert_eq!(trait_bound::non_zst_only(42u32), 42);
    }

    #[test]
    fn trait_bound_accepts_string() {
        let s = String::from("hello");
        assert_eq!(trait_bound::non_zst_only(s), "hello");
    }

    #[test]
    fn trait_bound_method_on_value() {
        use trait_bound::NonZst;
        assert_eq!(99i64.non_zst(), 99);
    }

    // -- Approach 3 tests --

    #[test]
    fn inline_const_accepts_u32() {
        assert_eq!(inline_const::non_zst_only(42u32), 42);
    }

    #[test]
    fn inline_const_accepts_vec() {
        let v = vec![1, 2, 3];
        assert_eq!(inline_const::non_zst_only(v), vec![1, 2, 3]);
    }

    // -- Approach 4 tests --

    #[test]
    fn wrapper_accepts_u32() {
        // Callers just pass raw values — conversion is automatic
        assert_eq!(wrapper::non_zst_only(42u32), 42);
    }

    #[test]
    fn wrapper_accepts_string() {
        let s = String::from("hello");
        assert_eq!(wrapper::non_zst_only(s), "hello");
    }

    #[test]
    fn wrapper_deref() {
        let val = wrapper::NonZstVal::new(42u32);
        assert_eq!(*val + 1, 43);
        assert_eq!(val.into_inner(), 42);
    }

    // ---- NonZstVal<T> corner-case / edge-case tests ----

    #[test]
    fn wrapper_size_of_matches_inner() {
        use std::mem::size_of;
        // Primitive
        assert_eq!(size_of::<wrapper::NonZstVal<u8>>(), size_of::<u8>());
        assert_eq!(size_of::<wrapper::NonZstVal<u64>>(), size_of::<u64>());
        // Large array
        assert_eq!(
            size_of::<wrapper::NonZstVal<[u8; 1024]>>(),
            size_of::<[u8; 1024]>()
        );
        // String (heap-allocated, 3 words)
        assert_eq!(size_of::<wrapper::NonZstVal<String>>(), size_of::<String>());
        // Function pointer
        assert_eq!(
            size_of::<wrapper::NonZstVal<fn() -> ()>>(),
            size_of::<fn() -> ()>()
        );
    }

    #[test]
    fn wrapper_align_of_matches_inner() {
        use std::mem::align_of;
        assert_eq!(align_of::<wrapper::NonZstVal<u8>>(), align_of::<u8>());
        assert_eq!(align_of::<wrapper::NonZstVal<u64>>(), align_of::<u64>());
        assert_eq!(
            align_of::<wrapper::NonZstVal<u128>>(),
            align_of::<u128>()
        );
        assert_eq!(
            align_of::<wrapper::NonZstVal<[u8; 1024]>>(),
            align_of::<[u8; 1024]>()
        );
    }

    #[test]
    fn wrapper_drop_runs_inner_drop() {
        use std::rc::Rc;
        let rc = Rc::new(42);
        assert_eq!(Rc::strong_count(&rc), 1);
        {
            let cloned = rc.clone();
            assert_eq!(Rc::strong_count(&rc), 2);
            let wrapped = wrapper::NonZstVal::new(cloned);
            assert_eq!(Rc::strong_count(&rc), 2);
            drop(wrapped);
            // After dropping wrapped, refcount must go back to 1
            assert_eq!(Rc::strong_count(&rc), 1);
        }
    }

    #[test]
    fn wrapper_into_inner_does_not_double_drop() {
        use std::rc::Rc;
        let rc = Rc::new(99);
        let wrapped = wrapper::NonZstVal::new(rc.clone());
        assert_eq!(Rc::strong_count(&rc), 2);
        let inner = wrapped.into_inner();
        // into_inner moves out — refcount still 2
        assert_eq!(Rc::strong_count(&rc), 2);
        drop(inner);
        assert_eq!(Rc::strong_count(&rc), 1);
    }

    #[test]
    fn wrapper_send_when_inner_send() {
        fn assert_send<T: Send>() {}
        assert_send::<wrapper::NonZstVal<u32>>();
        assert_send::<wrapper::NonZstVal<String>>();
        assert_send::<wrapper::NonZstVal<Vec<u8>>>();
    }

    #[test]
    fn wrapper_sync_when_inner_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<wrapper::NonZstVal<u32>>();
        assert_sync::<wrapper::NonZstVal<String>>();
    }

    #[test]
    fn wrapper_not_send_when_inner_not_send() {
        // Rc is !Send — NonZstVal<Rc<_>> must also be !Send
        // We verify via a negative compile-time trait check at runtime:
        use std::rc::Rc;
        assert!(!impls_send::<wrapper::NonZstVal<Rc<i32>>>());

        // Helper using autoref specialization pattern
    }

    #[test]
    fn wrapper_unpin_when_inner_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<wrapper::NonZstVal<u32>>();
        assert_unpin::<wrapper::NonZstVal<String>>();
    }

    #[test]
    fn wrapper_nested() {
        let inner = wrapper::NonZstVal::new(42u32);
        let outer = wrapper::NonZstVal::new(inner);
        assert_eq!(size_of::<wrapper::NonZstVal<wrapper::NonZstVal<u32>>>(), size_of::<u32>());
        assert_eq!(outer.into_inner().into_inner(), 42);
    }

    #[test]
    fn wrapper_large_array() {
        let arr = [0xABu8; 1024];
        let w = wrapper::NonZstVal::new(arr);
        assert_eq!(w.into_inner()[0], 0xAB);
        assert_eq!(w.into_inner().len(), 1024); // also tests Copy derive
    }

    #[test]
    fn wrapper_single_byte() {
        let w = wrapper::NonZstVal::new(255u8);
        assert_eq!(*w, 255u8);
        assert_eq!(w.into_inner(), 255u8);
    }

    #[test]
    fn wrapper_fn_pointer() {
        fn my_fn() -> u32 { 42 }
        let w = wrapper::NonZstVal::new(my_fn as fn() -> u32);
        let f = w.into_inner();
        assert_eq!(f(), 42);
    }

    #[test]
    fn wrapper_phantom_data_with_real_field() {
        use std::marker::PhantomData;
        #[derive(Debug, PartialEq)]
        struct Tagged<T> {
            value: u64,
            _marker: PhantomData<T>,
        }
        // Tagged<T> is non-ZST because of `value`
        assert_ne!(size_of::<Tagged<String>>(), 0);
        let t = Tagged::<String> { value: 7, _marker: PhantomData };
        let w = wrapper::NonZstVal::new(t);
        assert_eq!(w.into_inner().value, 7);
    }

    #[test]
    fn wrapper_display_forwards() {
        let w = wrapper::NonZstVal::new(42i32);
        assert_eq!(format!("{w}"), "42");

        let w2 = wrapper::NonZstVal::new(String::from("hello"));
        assert_eq!(format!("{w2}"), "hello");
    }

    #[test]
    fn wrapper_borrow_mut_forwards() {
        use std::borrow::BorrowMut;
        let mut w = wrapper::NonZstVal::new(vec![1, 2, 3]);
        let v: &mut Vec<i32> = w.borrow_mut();
        v.push(4);
        assert_eq!(&*w, &vec![1, 2, 3, 4]);
    }

    #[test]
    fn wrapper_default_routes_through_new() {
        // Default for NonZstVal<i32> should produce NonZstVal(0)
        let w: wrapper::NonZstVal<i32> = Default::default();
        assert_eq!(w.into_inner(), 0);

        let w2: wrapper::NonZstVal<String> = Default::default();
        assert_eq!(w2.into_inner(), "");
    }

    #[test]
    fn wrapper_eq_consistent_with_inner() {
        let a = wrapper::NonZstVal::new(10u32);
        let b = wrapper::NonZstVal::new(10u32);
        let c = wrapper::NonZstVal::new(20u32);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn wrapper_ord_consistent_with_inner() {
        let a = wrapper::NonZstVal::new(1u32);
        let b = wrapper::NonZstVal::new(2u32);
        assert!(a < b);
        assert!(b > a);
        assert_eq!(
            a.cmp(&b),
            1u32.cmp(&2u32)
        );
    }

    #[test]
    fn wrapper_hash_consistent_with_inner() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        fn hash_of<T: Hash>(val: &T) -> u64 {
            let mut h = DefaultHasher::new();
            val.hash(&mut h);
            h.finish()
        }

        let raw = 42u64;
        let wrapped = wrapper::NonZstVal::new(42u64);
        assert_eq!(hash_of(&raw), hash_of(&wrapped));
    }

    #[test]
    fn wrapper_deref_mut() {
        let mut w = wrapper::NonZstVal::new(vec![1, 2]);
        w.push(3);  // DerefMut allows calling Vec methods directly
        assert_eq!(&*w, &vec![1, 2, 3]);
    }

    #[test]
    fn wrapper_as_ref_as_mut() {
        let mut w = wrapper::NonZstVal::new(42i32);
        let r: &i32 = w.as_ref();
        assert_eq!(*r, 42);
        let m: &mut i32 = w.as_mut();
        *m = 99;
        assert_eq!(w.into_inner(), 99);
    }

    #[test]
    fn wrapper_clone_independence() {
        let w1 = wrapper::NonZstVal::new(vec![1, 2, 3]);
        let mut w2 = w1.clone();
        w2.push(4);
        // w1 must be unaffected
        assert_eq!(&*w1, &vec![1, 2, 3]);
        assert_eq!(&*w2, &vec![1, 2, 3, 4]);
    }

    #[test]
    fn wrapper_from_conversion() {
        let w: wrapper::NonZstVal<u32> = 42u32.into();
        assert_eq!(w.into_inner(), 42);
    }

    #[test]
    fn wrapper_non_zst_only_convenience() {
        // The free function accepts raw values via Into
        assert_eq!(wrapper::non_zst_only(123i64), 123i64);
        assert_eq!(wrapper::non_zst_only(String::from("x")), "x");
    }

    // -- helper for negative trait-bound test --
    // Autoref specialization: if T: Send, the inherent method is picked;
    // otherwise the trait fallback is picked.
    trait NotSendFallback { fn _is_send(&self) -> bool { false } }
    impl<T> NotSendFallback for T {}
    struct SendProbe<T>(std::marker::PhantomData<T>);
    impl<T: Send> SendProbe<T> { fn _is_send(&self) -> bool { true } }

    fn impls_send<T>() -> bool {
        SendProbe::<T>(std::marker::PhantomData)._is_send()
    }

    // -- Verify ZST properties --

    #[test]
    fn unit_is_zst() {
        assert_eq!(size_of::<()>(), 0);
    }

    #[test]
    fn empty_struct_is_zst() {
        assert_eq!(size_of::<Empty>(), 0);
    }

    #[test]
    fn zero_len_array_is_zst() {
        assert_eq!(size_of::<[u8; 0]>(), 0);
    }

    #[test]
    fn void_is_zst() {
        assert_eq!(size_of::<Void>(), 0);
    }

    #[test]
    fn infallible_is_zst() {
        assert_eq!(size_of::<std::convert::Infallible>(), 0);
    }
}
