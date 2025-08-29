# Conditional Linking with cfg(test)

## Overview

This project demonstrates **conditional linking** in Rust - how to link different C libraries depending on whether the code is built in production mode (`cargo build`/`cargo run`) or test mode (`cargo test`).

## What This Project Does

- **Production Mode**: Links to `libfoo.a` and calls production functions
- **Test Mode**: Links to `libtestfoo.a` and calls test-specific functions
- Uses `#[cfg(test)]` and `#[cfg(not(test))]` attributes to control which library is linked

## Key Files

```plaintext
src/
├── main.rs          # Main Rust code with conditional extern blocks
├── libfoo.c         # Production library implementation
└── libtestfoo.c     # Test library implementation
build.rs             # Build script that compiles both C libraries
```

## How It Works

### 1. Build Script (`build.rs`)

Compiles both C libraries during the build process:

```rust
// Build libfoo (production library)
cc::Build::new()
    .file("src/libfoo.c")
    .cargo_metadata(false)  // CRITICAL: Prevents automatic linking
    .compile("foo");

// Build libtestfoo (test library) 
cc::Build::new()
    .file("src/libtestfoo.c")
    .cargo_metadata(false)  // CRITICAL: Prevents automatic linking
    .compile("testfoo");
```

**⚠️ Important**: The `cargo_metadata(false)` setting is **essential** because:

- Build scripts cannot be conditionally executed based on test vs production builds
- Without this, the `cc` crate would automatically emit `cargo:rustc-link-lib` instructions for both libraries
- This would cause both libraries to be linked simultaneously, breaking our conditional linking
- Instead, we let the `#[cfg]` attributes in `main.rs` control which library gets linked

### 2. Conditional FFI Bindings (`main.rs`)

Uses conditional compilation to link different libraries with **empty extern blocks** for linking and a separate extern block for function declarations:

```rust
// Empty extern blocks with #[link] attributes for conditional linking
#[cfg(not(test))]
#[link(name = "foo")]
unsafe extern "C" {
    // Empty - the #[link] attribute handles library linking
}

#[cfg(test)]
#[link(name = "testfoo")]
unsafe extern "C" {
    // Empty - the #[link] attribute handles library linking
}

// Shared extern block for function declarations
unsafe extern "C" {
    fn print_library_info();
    
    #[cfg(not(test))]
    fn foo_specific_function();
    
    #[cfg(test)]
    fn testfoo_specific_function();
}
```

**Key Design**:

- **Empty extern blocks** with `#[link]` attributes control which library is linked
- **Separate extern block** contains the actual function declarations
- Both approaches can coexist because linking happens globally

### 3. C Library Implementations

**Production (`libfoo.c`)**:

```c
void print_library_info() {
    printf("libfoo\n");
}

void foo_specific_function() {
    printf("This is from libfoo specifically\n");
}
```

**Test (`libtestfoo.c`)**:

```c
void print_library_info() {
    printf("libtestfoo\n");
}

void testfoo_specific_function() {
    printf("This is from libtestfoo specifically\n");
}
```

## Usage

### Production Mode (`cargo run`)

```powershell
PS C:\project> cargo run
warning: test-different-link-per-block@0.1.0: Libraries built in: C:\...\target\debug\build\...\out
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.51s
     Running `target\debug\test-different-link-per-block.exe`
Testing conditional linking with cfg_if:
  - cargo build links to libfoo
  - cargo test links to libtestfoo

print_library_info:
libfoo
foo_specific_function:
This is from libfoo specifically
```

**Analysis**: Production mode links with `foo.lib` and calls `foo_specific_function()`

### Test Mode (`cargo test`)

```powershell
PS C:\project> cargo test
warning: test-different-link-per-block@0.1.0: Libraries built in: C:\...\target\debug\build\...\out
   Compiling test-different-link-per-block v0.1.0 (C:\Users\melvinwang\test-different-link-per-block)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.00s
     Running unittests src\main.rs (target\debug\deps\test_different_link_per_block-dbddbff4eb21ae34.exe)

running 1 test
Testing conditional linking with cfg_if:
  - cargo build links to libfoo
  - cargo test links to libtestfoo

print_library_info:
libtestfoo
testfoo_specific_function:
This is from libtestfoo specifically
test tests::test_conditional_linking ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

**Analysis**: Test mode links with `testfoo.lib` and calls `testfoo_specific_function()`

### Key Output Differences

| Mode                         | Linked Library | Function Called               | Library Output                                        |
| ---------------------------- | -------------- | ----------------------------- | ----------------------------------------------------- |
| **Production** (`cargo run`) | `foo.lib`      | `foo_specific_function()`     | "libfoo" + "This is from libfoo specifically"         |
| **Test** (`cargo test`)      | `testfoo.lib`  | `testfoo_specific_function()` | "libtestfoo" + "This is from libtestfoo specifically" |

## Technical Details

### Linker Analysis

- **Production**: Links with `foo.lib` from OUT_DIR
- **Test**: Links with `testfoo.lib` from OUT_DIR
- Conditional compilation attributes (`#[cfg(test)]`) control which `extern` blocks are active
- Build script ensures both libraries are always built but only the appropriate one is linked

### Link Attribute Behavior: Empty Extern Blocks as Link Instructions

The `#[link(name = "...")]` attribute on extern blocks functions **identically** to `rustc-link-arg` because:

**Key Insight**: The linker is invoked only **once per compilation unit** and attempts to resolve **all** specified libraries, regardless of which extern block they're associated with.

```rust
// These two approaches are functionally equivalent:

// Approach 1: Link attribute on extern block
#[cfg(test)]
#[link(name = "testfoo")]
extern "C" {
    // Functions can be empty - the #[link] attribute still takes effect
}

// Approach 2: Direct rustc link argument (equivalent behavior)
// cargo rustc -- -l testfoo
```

**Why Empty Extern Blocks Work**:

1. **Linker Invocation**: The linker processes all `-l` flags at once, not per-symbol
2. **Library Resolution**: `#[link(name = "testfoo")]` translates to `-l testfoo` linker flag
3. **Symbol Independence**: Library linking happens **before** symbol resolution
4. **Global Effect**: Once a library is linked, all its symbols are available throughout the compilation unit

**Practical Implications**:

```rust
// Even with empty extern blocks, the library is fully linked
#[cfg(test)]
#[link(name = "testfoo")]
extern "C" {
    // Empty block - but testfoo.lib is still linked and all symbols available
}

// This works because the linker has already processed testfoo.lib
unsafe fn call_any_testfoo_symbol() {
    // All symbols from testfoo.lib are available, not just those declared in extern blocks
    let symbol = std::mem::transmute::<*const (), extern "C" fn()>(
        GetProcAddress(LoadLibrary("testfoo"), "any_symbol")
    );
    symbol();
}
```

**Linker Processing Order**:

1. **Compilation**: Rust compiles source, collects all `#[link]` attributes
2. **Link Flag Generation**: Each `#[link(name = "X")]` becomes `-l X`
3. **Single Linker Invocation**: Linker called once with all flags: `rustc ... -l foo -l bar -l testfoo`
4. **Global Library Loading**: All specified libraries loaded into final binary
5. **Symbol Resolution**: Extern function calls resolved against all loaded libraries

### Conditional Compilation Benefits

1. **Test Isolation**: Test code can use mock implementations without affecting production
2. **Different Behavior**: Same function signatures can have completely different implementations
3. **Compile-Time Safety**: No runtime overhead - linking decision made at compile time
4. **Single Codebase**: One set of function signatures works for both scenarios

## Build System Integration

The `cc` crate integration allows seamless C library compilation with careful control over linking:

- Libraries built in `OUT_DIR` during build script execution
- **`cargo_metadata(false)` is crucial**: Prevents the `cc` crate from automatically emitting `cargo:rustc-link-lib` instructions
- `cargo:rustc-link-search=native={}` tells rustc where to find the libraries (but doesn't link them)
- `#[link(name = "...")]` attributes in conditional `extern` blocks control actual linking
- Build script runs for both production and test builds, but linking is controlled at compile time

### Why `cargo_metadata(false)` is Essential

Without this setting, the build script would automatically link both libraries:

```rust
// ❌ BAD: This would emit cargo:rustc-link-lib=foo AND cargo:rustc-link-lib=testfoo
cc::Build::new().file("src/libfoo.c").compile("foo");     // Links automatically
cc::Build::new().file("src/libtestfoo.c").compile("testfoo"); // Links automatically
```

With `cargo_metadata(false)`, we get manual control:

```rust
// ✅ GOOD: Only compiles libraries, doesn't link them
cc::Build::new().file("src/libfoo.c").cargo_metadata(false).compile("foo");
cc::Build::new().file("src/libtestfoo.c").cargo_metadata(false).compile("testfoo");
// Conditional #[cfg] attributes control which one gets linked
```

## Use Cases

This pattern is useful for:

- **Testing with Mocks**: Replace production C libraries with test implementations
- **Hardware Abstraction**: Different implementations for different hardware targets
- **Feature Flags**: Link different library versions based on cargo features
- **Platform-Specific Code**: Different native libraries per operating system

## Key Insights

- **Conditional Linking Works**: Unlike symbol-level linking, library-level conditional linking is fully supported
- **Compile-Time Decision**: Library selection happens at compile time, not runtime
- **Same Interface**: Both libraries can expose identical function signatures
- **Build Script Flexibility**: Complex linking scenarios can be handled in `build.rs`
