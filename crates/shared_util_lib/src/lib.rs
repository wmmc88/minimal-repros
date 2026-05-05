//! `shared_util_lib`
//!
//! Defines a single TraceLogging provider and an API for emitting a
//! driver-type-tagged "DriverInit" event. The driver-type string is selected
//! at compile time by the `driver_model__driver_type` cfg flag emitted by
//! `wdk-build`.
//!
//! When the `kernel_mode` feature is enabled, this crate forwards it to
//! `tracelogging`, which switches its native ETW calls from `EventXxx`
//! (user-mode, links against `OneCore_apiset`) to `EtwXxx` (kernel-mode,
//! resolved against the kernel image at link time).

#![no_std]

use tracelogging as tlg;

const DRIVER_TYPE_NAME: &str = ::core::cfg_select! {
    driver_model__driver_type = "WDM" => "WDM",
    driver_model__driver_type = "KMDF" => "KMDF",
    driver_model__driver_type = "UMDF" => "UMDF",
    _ => "Unknown",
};

tlg::define_provider!(MIXED_PROVIDER, "MixedTraceLogging");

/// Register the static TraceLogging provider.
///
/// # Safety
/// `unregister_provider` MUST be called before the driver image is unloaded.
pub unsafe fn register_provider() -> u32 {
    // SAFETY: caller upholds the unregister-before-unload invariant.
    unsafe { MIXED_PROVIDER.register() }
}

/// Unregister the static TraceLogging provider.
pub fn unregister_provider() -> u32 {
    MIXED_PROVIDER.unregister()
}

/// Emit a `DriverInit` event tagged with the driver type the consuming driver
/// is configured for. Different driver models will produce different events,
/// driven by the `driver_model__driver_type` cfg.
pub fn log_driver_init() {
    let _ = tlg::write_event!(
        MIXED_PROVIDER,
        "DriverInit",
        level(Informational),
        str8("driver_type", DRIVER_TYPE_NAME),
    );
}

/// Returns the compile-time-selected driver-type string for diagnostics.
#[must_use]
pub const fn driver_type_name() -> &'static str {
    DRIVER_TYPE_NAME
}
