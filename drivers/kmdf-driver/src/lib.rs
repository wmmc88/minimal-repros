//! KMDF driver of the `mixed-tracelogging` repro.
//!
//! `DriverEntry` lives in [`main_shared_lib`] and is wired into this cdylib
//! by the `/ENTRY:DriverEntry` linker directive that
//! `wdk_build::configure_wdk_binary_build` emits in the build script.
//!
//! This crate only contributes the kernel-mode panic handler and global
//! allocator — both of which must be linked into the binary crate itself.

#![no_std]

#[cfg(not(test))]
extern crate wdk_panic;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: wdk_alloc::WdkAllocator = wdk_alloc::WdkAllocator;

// Reference `main_shared_lib` to ensure Cargo links its rlib (and therefore
// its `DriverEntry` symbol) into this cdylib.
use main_shared_lib as _;
