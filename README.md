# Disallowing Zero-Sized Types in Generic Functions

> **Best stable approach: `NonZstVal<T>` (Approach 4).**
> Wrap values in a `#[repr(transparent)]` newtype whose only constructors contain a const assertion rejecting ZSTs.
> Functions accept `impl Into<NonZstVal<T>>` so callers pass raw values and the check fires automatically.
> See the [`wrapper` module in `src/lib.rs`](src/lib.rs) for the implementation.

## Problem statement

Some generic APIs are only meaningful for types that carry data — FFI buffers, serialization layers, memory-mapped I/O, typed arenas, etc.
Accepting a ZST (like `()`, `Empty`, `[u8; 0]`, or an uninhabited enum) in those contexts is at best a silent no-op and at worst undefined behavior.
Rust has no built-in `T: !ZeroSized` bound, so library authors must encode the constraint themselves.
This repo compares four approaches, from nightly-only to fully stable, for rejecting ZSTs at compile time.

## Comparison

| # | Approach | Rust version | Where the check lives | Automatic? | Visible in signature? | Downsides |
|---|----------|-------------|----------------------|------------|----------------------|-----------|
| 1 | Const generic where-clause | **nightly** | `where` clause | ✅ Yes | ✅ Yes | Requires `generic_const_exprs` (incomplete, known ICEs) |
| 2 | Sealed trait bound | stable | Associated const + `.non_zst()` call | ❌ Must call `.non_zst()` | ✅ `T: NonZst` | Bound alone is inert — assertion only fires when `ASSERT` is referenced |
| 3 | Inline const assertion | stable | Function body | ✅ Yes | ❌ No | Hidden from callers, only fires during monomorphization |
| 4 | **Newtype wrapper** ⭐ | stable | `From<T>` / `new()` constructor | ✅ Yes | ✅ `Into<NonZstVal<T>>` | Callers see `NonZstVal` in the type (a pro for documentation) |

## Detailed breakdown

### Approach 1: Const generic where-clause (nightly)

**How it works.** A helper struct `Assert<const B: bool>` implements a marker trait `IsTrue` only for `Assert<true>`. The function adds a where-clause `Assert<{ size_of::<T>() != 0 }>: IsTrue`, which the compiler evaluates as a const expression during trait resolution.

```rust
pub fn non_zst_only<T>(value: T) -> T
where
    Assert<{ size_of::<T>() != 0 }>: IsTrue,
{ value }
```

**Pros:**
- Truly automatic — no call sites to remember.
- Clean "trait bound not satisfied" error.
- Constraint is visible in the function signature.

**Cons:**
- Requires nightly `#![feature(generic_const_exprs)]` ([tracking issue #76560](https://github.com/rust-lang/rust/issues/76560)).
- Marked `incomplete_features` with known ICEs — far from stabilization.
- Conflicts with inline `const {}` blocks in the same crate (requires extracting the assertion into a `const fn` workaround).

---

### Approach 2: Sealed trait bound

**How it works.** A sealed `NonZst` trait has a blanket impl for all `T: Sized`. An associated constant `ASSERT` contains `assert!(size_of::<Self>() != 0)`, and a default method `.non_zst()` references that constant so calling it forces evaluation.

```rust
pub fn non_zst_only<T: NonZst>(value: T) -> T {
    value.non_zst()
}
```

**Pros:**
- Visible in the signature as `T: NonZst`.
- Sealed — external code cannot bypass the check.

**Cons:**
- The bound `T: NonZst` alone does **not** reject ZSTs. You must call `.non_zst()` or explicitly reference `Self::ASSERT` somewhere. If you forget, ZSTs slip through silently.
- See [Key insight about associated constants](#key-insight-about-associated-constants) below for why.

---

### Approach 3: Inline const assertion

**How it works.** A `const {}` block inside the function body evaluates `assert!(size_of::<T>() != 0)` at compile time when the function is monomorphized.

```rust
pub fn non_zst_only<T>(value: T) -> T {
    const { assert_non_zst::<T>() }
    value
}
```

**Pros:**
- Simplest — no trait boilerplate, no wrapper type.
- Works on stable Rust.

**Cons:**
- Hidden in the function body — callers have no idea the function rejects ZSTs without reading the source.
- Only fires during monomorphization (not `cargo check` alone — requires a concrete instantiation).

---

### Approach 4: Newtype wrapper (⭐ RECOMMENDED)

**How it works.** `NonZstVal<T>` is a `#[repr(transparent)]` newtype around `T`. Both `NonZstVal::new()` and `From<T>` contain the const assertion. Functions accept `impl Into<NonZstVal<T>>`, so callers pass raw values and the ZST check fires automatically during the `From` conversion.

```rust
pub fn non_zst_only<T>(value: impl Into<NonZstVal<T>>) -> T {
    value.into().into_inner()
}

// Caller — just pass a raw value:
non_zst_only(42u32);
```

**Pros:**
- Check is automatic via `From<T>` — no manual calls to remember.
- Constraint is visible in the signature (`Into<NonZstVal<T>>`).
- `#[repr(transparent)]` makes it zero-cost — same size and alignment as `T`.
- Rich trait surface makes it a drop-in replacement in most contexts.
- Structural guarantee: a `NonZstVal<T>` **cannot** hold a ZST.

**Cons:**
- Callers see `NonZstVal` in the type (which is actually a pro for documentation).
- Type inference edge cases in deeply generic forwarding code (narrow).

**Full trait surface:** `Clone`, `Copy`, `Debug`, `Display`, `PartialEq`/`Eq`, `PartialOrd`/`Ord`, `Hash`, `Default`, `Deref`/`DerefMut`, `AsRef`/`AsMut`, `Borrow`/`BorrowMut`, `From`.

## Running the tests

```sh
cargo test                                    # stable (approaches 2, 3, 4)
cargo +nightly test --features nightly        # all approaches
```

## Key insight about associated constants

In Rust, associated constants with default values are **lazily evaluated** — the compiler only checks them when they are explicitly referenced (e.g., `let () = Self::ASSERT;`).
This is why `T: NonZst` alone (approach 2) doesn't fire the assertion: the bound is satisfied by the blanket impl, but the `ASSERT` constant inside it is never evaluated unless something in the code forces it.
This is a fundamental language behavior, not a bug — it means that "assertion-in-associated-const" patterns always require a manual trigger point.

## Links

- `generic_const_exprs` tracking issue: <https://github.com/rust-lang/rust/issues/76560>
