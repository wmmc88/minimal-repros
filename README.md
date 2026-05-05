# mixed-tracelogging

Minimal repro showing how to share a `tracelogging`-using library across
WDM, KMDF, and UMDF Rust drivers built with
[`windows-drivers-rs`](https://github.com/microsoft/windows-drivers-rs), with
the `tracelogging/kernel_mode` feature enabled **only** for the kernel-mode
driver variants.

## What this demonstrates

- A chained pair of shared library crates (`main_shared_lib` →
  `shared_util_lib` → `tracelogging`) that can be consumed by any of the three
  Windows driver models.
- `shared_util_lib` uses [`core::cfg_select!`] against the
  `cfg(driver_model__driver_type = "...")` flag emitted by `wdk-build` to
  select a different driver-type string at compile time, so the same call
  site emits a different `DriverInit` TraceLogging event for each driver
  model.
- `main_shared_lib` defines the actual `#[unsafe(export_name = "DriverEntry")]
  pub unsafe extern "system" fn driver_entry(...)` symbol. Driver crates do
  NOT need to define their own `DriverEntry`: `wdk-build`'s
  `configure_wdk_binary_build()` emits a
  `cargo::rustc-cdylib-link-arg=/ENTRY:DriverEntry` directive in each
  driver's build script, and the linker pulls the `DriverEntry` symbol from
  whichever rlib in the link graph defines it — in this repro, that's
  `main_shared_lib`. Inside the function, [`core::cfg_select!`] picks the
  UMDF behavior (register, log, unregister inline) vs the KMDF/WDM behavior
  (register, log, install a `DriverUnload` callback), again driven by
  `cfg(driver_model__driver_type = "...")`.
- Both shared library crates carry a `wdk_build` build script that emits the
  required `rustc-check-cfg` directive for `driver_model__driver_type` and
  best-effort calls `wdk_build::configure_wdk_library_build()` so the cfg is
  set to the correct value for the consuming driver's metadata.
- The `tracelogging/kernel_mode` Cargo feature is enabled **only** in the
  KMDF and WDM driver builds. Each kernel-mode driver crate declares a
  direct dependency on `tracelogging` with `features = ["kernel_mode"]`,
  purely as a way to feed the feature into Cargo's per-workspace feature
  resolver. Cargo then unifies that activation with the same `tracelogging`
  instance that `shared_util_lib` depends on, so the kernel-mode `EtwXxx`
  ETW APIs are linked. The UMDF driver does NOT declare `tracelogging`
  directly, so the only depender is `shared_util_lib` (no `kernel_mode`),
  and the user-mode `EventXxx` ETW APIs (`OneCore_apiset`) are linked
  instead.
- The shared library crates intentionally do NOT expose a `kernel_mode`
  Cargo feature themselves. They depend on `tracelogging` with
  `default-features = false, features = ["etw", "macros"]` and stay
  agnostic to the user-mode/kernel-mode choice.

[`core::cfg_select!`]: https://doc.rust-lang.org/core/macro.cfg_select.html

## Layout

```
mixed-tracelogging/
├── Cargo.toml                top-level workspace (members = the two shared crates)
├── crates/
│   ├── shared_util_lib/      no_std lib; defines the static TraceLogging provider
│   │                         and a cfg_select!-gated driver-type string. Has its own
│   │                         build.rs that calls wdk_build::configure_wdk_library_build.
│   └── main_shared_lib/      no_std facade; defines the DriverEntry symbol with
│                             cfg_select! inside picking UMDF vs KMDF/WDM behavior.
│                             Also has a wdk-build build.rs.
└── drivers/                  each driver is its OWN standalone Cargo workspace
    ├── kmdf-driver/          panic_handler + global_allocator + use main_shared_lib as _
    ├── umdf-driver/          just `use main_shared_lib as _;`
    └── wdm-driver/           same as kmdf-driver
```

The driver crates do not contain a `DriverEntry` function themselves. The
`/ENTRY:DriverEntry` linker arg emitted by `wdk-build` for each driver
build pulls the `DriverEntry` symbol from `main_shared_lib`'s rlib. The
`use main_shared_lib as _;` line ensures Cargo links `main_shared_lib`'s
rlib into the cdylib so the linker can find the symbol; for kernel-mode
drivers, the `extern crate wdk_panic;` and `#[global_allocator] static`
must live in the binary crate itself for the panic handler / allocator
wiring to apply, and they must NOT be present in the UMDF driver, which
uses `std`.

The drivers are intentionally NOT members of the top-level workspace. If they
were, Cargo's per-workspace feature unification would force
`tracelogging/kernel_mode` ON for the UMDF build too, defeating the entire
point of this repro. Each driver therefore has its own empty `[workspace]`
stanza so it forms its own standalone Cargo workspace with its own
`Cargo.lock`.

## Prerequisites

- Rust **stable 1.95 or newer** (the
  [`core::cfg_select!`](https://doc.rust-lang.org/core/macro.cfg_select.html)
  macro was stabilized in 1.95).
- An [eWDK developer prompt](https://learn.microsoft.com/en-us/windows-hardware/drivers/develop/using-the-enterprise-wdk#getting-started)
  with `WDKContentRoot` set in the environment.
- LLVM 17 on `PATH` (`bindgen` requirement; LLVM 18 has an ARM64 bindgen
  regression — see the windows-drivers-rs README for details).
- `cargo-wdk` installed: `cargo install cargo-wdk`.

## Building

The canonical command is to run `cargo wdk build --sample` from the
`drivers/` directory. Because `drivers/` does not contain a `Cargo.toml`,
`cargo-wdk` falls into its **emulated workspace** mode and iterates each
immediate subdirectory containing a `Cargo.toml`, building each driver in
its own independent workspace:

```pwsh
cd drivers
cargo wdk build --sample
```

The `--sample` flag is required because the `.inx` files in this repro use
`Class = Sample` (`ClassGuid = {78A1C341-4539-11d3-B88D-00C04FAD5171}`),
which is reserved and triggers an `infverif` failure unless `--sample` is
passed.

To build a single driver:

```pwsh
cd drivers\kmdf-driver
cargo wdk build --sample
```

A signed driver package is emitted under
`drivers\<driver>\target\<profile>\<driver_name>_package` (with hyphens in
the crate name converted to underscores in the package directory name).

The two shared library crates can be type-checked from the repository root
without a WDK environment:

```pwsh
cargo check
```

When the shared libs are built outside of a driver context (no
`[package.metadata.wdk.driver-model]` reachable), the
`driver_model__driver_type` cfg is unset and `shared_util_lib` falls back to
emitting `"Unknown"` for the driver type. This is intentional so that
`cargo check` at the repo root succeeds without an eWDK prompt.

## Verifying the kernel_mode feature is correctly gated

The `cargo tree -e features -i tracelogging` invocation will show which
features are activated for `tracelogging` in each driver's build graph. Run
it inside each driver folder:

```pwsh
cd drivers\kmdf-driver ; cargo tree -e features -i tracelogging --target x86_64-pc-windows-msvc
cd drivers\wdm-driver  ; cargo tree -e features -i tracelogging --target x86_64-pc-windows-msvc
cd drivers\umdf-driver ; cargo tree -e features -i tracelogging --target x86_64-pc-windows-msvc
```

Expected:

- `kmdf-driver` and `wdm-driver` show `tracelogging feature "kernel_mode"`
  in the tree, sourced directly from the driver crate (which declares
  `tracelogging = { ..., features = ["kernel_mode"] }`). Because Cargo
  unifies features per workspace, the same `tracelogging` instance that
  `shared_util_lib` uses also gets `kernel_mode` activated.
- `umdf-driver` does NOT show `tracelogging feature "kernel_mode"`.
  The only depender on `tracelogging` in the UMDF build is
  `shared_util_lib`, which doesn't request `kernel_mode`.

Because each driver is a standalone workspace, feature resolution is fully
isolated per driver, so the UMDF build never sees the kernel-mode ETW APIs.
