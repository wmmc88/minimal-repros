//! Build script for the `umdf-driver` cdylib.
//!
//! `wdk_build::configure_wdk_binary_build()` reads the
//! `[package.metadata.wdk.driver-model]` table from this crate's `Cargo.toml`,
//! emits the `driver_model__driver_type="UMDF"` cfg, and configures linker
//! flags appropriate for a UMDF driver.

fn main() -> Result<(), wdk_build::ConfigError> {
    wdk_build::configure_wdk_binary_build()
}
