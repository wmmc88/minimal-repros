//! Build script for `main_shared_lib`.
//!
//! Same shape as `shared_util_lib`'s build script — the driver-model cfg
//! and its check-cfg are emitted by `wdk_build::configure_wdk_library_build()`,
//! which also handles the standalone (no-WDK-metadata) case with its own
//! warning.

fn main() -> Result<(), wdk_build::ConfigError> {
    wdk_build::configure_wdk_library_build()
}
