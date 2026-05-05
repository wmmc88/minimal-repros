//! UMDF driver of the `mixed-tracelogging` repro.
//!
//! `DriverEntry` lives in [`main_shared_lib`] and is wired into this cdylib
//! by the `/ENTRY:DriverEntry` linker directive that
//! `wdk_build::configure_wdk_binary_build` emits in the build script. UMDF
//! runs in user mode and gets `std`'s panic handler and allocator
//! automatically, so this crate has nothing else to contribute.

// Reference `main_shared_lib` to ensure Cargo links its rlib (and therefore
// its `DriverEntry` symbol) into this cdylib.
use main_shared_lib as _;
