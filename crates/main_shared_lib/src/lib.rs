//! `main_shared_lib`
//!
//! Public driver-facing facade. Re-exports the TraceLogging API from
//! [`shared_util_lib`] and defines the `DriverEntry` symbol consumed by all
//! driver cdylibs in this repro.
//!
//! Driver crates do not need to define `DriverEntry` themselves: when
//! `wdk-build`'s `configure_wdk_binary_build()` runs in a driver crate's
//! build script, it emits the `cargo::rustc-cdylib-link-arg=/ENTRY:DriverEntry`
//! linker directive, and `link.exe` then pulls the `DriverEntry` symbol from
//! whichever rlib in the link graph defines it — in this repro, that's the
//! function below.

#![no_std]

pub use shared_util_lib::{
    driver_type_name,
    log_driver_init,
    register_provider,
    unregister_provider,
};

/// Unified `DriverEntry` for any of the supported driver models. This is the
/// `DriverEntry` symbol the kernel (or UMDF host) calls into for any driver
/// in this repro — driver crates only need to depend on `main_shared_lib`,
/// they do not need to define their own `DriverEntry`.
///
/// The model-specific behavior is selected at compile time via
/// [`core::cfg_select`] against the `driver_model__driver_type` cfg flag
/// emitted by `wdk-build`:
///
/// - For UMDF, the TraceLogging provider is registered, the `DriverInit`
///   event is emitted, and the provider is unregistered inline before
///   returning.
/// - For KMDF and WDM, the provider is registered, the event is emitted,
///   and a `DriverUnload` callback is installed on the supplied
///   `DRIVER_OBJECT` so the provider is unregistered before the driver image
///   is unloaded.
///
/// The function is itself cfg-gated to the three known driver models so the
/// standalone (no-WDK) `cargo check` of the shared workspace still compiles
/// (without WDK metadata, `wdk_sys` doesn't expose `DRIVER_OBJECT` /
/// `NTSTATUS` / etc).
///
/// # Safety
///
/// `driver` must be a valid `DRIVER_OBJECT` pointer for the consuming driver
/// model and `_registry_path` must point to a valid `UNICODE_STRING` (or be
/// null where permitted).
// SAFETY: "DriverEntry" is the required symbol name for Windows driver entry
// points. No other function in the link graph exports this name, preventing
// symbol conflicts.
#[cfg(any(
    driver_model__driver_type = "WDM",
    driver_model__driver_type = "KMDF",
    driver_model__driver_type = "UMDF",
))]
#[unsafe(export_name = "DriverEntry")]
pub unsafe extern "system" fn driver_entry(
    driver: wdk_sys::PDRIVER_OBJECT,
    _registry_path: wdk_sys::PCUNICODE_STRING,
) -> wdk_sys::NTSTATUS {
    // SAFETY: matched by either the inline unregister (UMDF arm below) or
    // the unregister inside the installed `kernel_mode_driver_unload`
    // callback (KMDF/WDM arm below).
    let _ = unsafe { register_provider() };
    log_driver_init();

    ::core::cfg_select! {
        driver_model__driver_type = "UMDF" => {
            let _ = driver;
            let _ = unregister_provider();
        }
        _ => {
            // SAFETY: the caller guarantees `driver` is a valid kernel-mode
            // `DRIVER_OBJECT` pointer (this arm only runs for KMDF and WDM).
            unsafe {
                (*driver).DriverUnload =
                    ::core::option::Option::Some(kernel_mode_driver_unload);
            }
        }
    }

    wdk_sys::STATUS_SUCCESS
}

#[cfg(any(
    driver_model__driver_type = "KMDF",
    driver_model__driver_type = "WDM",
))]
extern "C" fn kernel_mode_driver_unload(_driver: *mut wdk_sys::DRIVER_OBJECT) {
    let _ = unregister_provider();
}

