//! Build script for `shared_util_lib`.
//!
//! `wdk_build::configure_wdk_library_build()` reads the consuming driver's
//! `[package.metadata.wdk.driver-model]` (when this crate is built as a
//! transitive dep of a driver), emits the `driver_model__driver_type` cfg
//! plus the matching `rustc-check-cfg` directive, and gracefully handles the
//! standalone (no-WDK-metadata) case with its own warning.

fn main() -> Result<(), wdk_build::ConfigError> {
    wdk_build::configure_wdk_library_build()
}
