# Failure log

```
cargo clean && cargo test --tests
     Removed 420 files, 200.4MiB total
   Compiling proc-macro2 v1.0.103
   Compiling serde_core v1.0.228
   Compiling unicode-ident v1.0.22
   Compiling quote v1.0.42
   Compiling serde_json v1.0.145
   Compiling serde v1.0.228
   Compiling winnow v0.7.13
   Compiling prettyplease v0.2.37
   Compiling memchr v2.7.6
   Compiling toml_writer v1.0.4
   Compiling ryu v1.0.20
   Compiling itoa v1.0.15
   Compiling glob v0.3.3
   Compiling diff v0.1.13
   Compiling fastrand v2.3.0
   Compiling syn v2.0.111                                                                                                                                                                          
   Compiling toml_parser v1.0.4
   Compiling toml_datetime v0.7.3
   Compiling serde_spanned v1.0.3
   Compiling serde_derive v1.0.228
   Compiling toml v0.9.8
   Compiling macrotest v1.2.0
   Compiling foo v0.1.0 (D:\git-repos\github\minimal-repros)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 9.98s
     Running unittests src\lib.rs (target\debug\deps\foo-76344593462948b6.exe)

running 2 tests
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling proc-macro2 v1.0.103
    Checking unicode-ident v1.0.22
   Compiling quote v1.0.42
   Compiling proc-macro2 v1.0.103
    Checking unicode-ident v1.0.22
   Compiling quote v1.0.42
error: linking with `link.exe` failed: exit code: 1104
  |
  = note: "C:\\Program Files\\Microsoft Visual Studio\\2022\\Enterprise\\VC\\Tools\\MSVC\\14.44.35207\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "C:\\Users\\MELVIN~1\\AppData\\Local\\Temp\\rustc4hFlGk\\symbols.o" "<2 object files omitted>" "<sysroot>\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib/{libstd-*,libpanic_unwind-*,libcfg_if-*,libwindows_targets-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace_alloc-*,libunwind-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "kernel32.lib" "kernel32.lib" "kernel32.lib" "ntdll.lib" "userenv.lib" "ws2_32.lib" "dbghelp.lib" "/defaultlib:msvcrt" "/NXCOMPAT" "/OUT:D:\\git-repos\\github\\minimal-repros\\target\\tests\\macrotest\\debug\\build\\quote-4e19cce3fccb8eb3\\build_script_build-4e19cce3fccb8eb3.exe" "/OPT:REF,NOICF" "/DEBUG" "/PDBALTPATH:%_PDB%" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\liballoc.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libstd.natvis"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: LINK : fatal error LNK1104: cannot open file 'D:\git-repos\github\minimal-repros\target\tests\macrotest\debug\build\quote-4e19cce3fccb8eb3\build_script_build-4e19cce3fccb8eb3.exe'      
error: could not compile `quote` (build script) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
error: linking with `link.exe` failed: exit code: 1104
  |
  = note: "C:\\Program Files\\Microsoft Visual Studio\\2022\\Enterprise\\VC\\Tools\\MSVC\\14.44.35207\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "C:\\Users\\MELVIN~1\\AppData\\Local\\Temp\\rustc6piSy7\\symbols.o" "<3 object files omitted>" "<sysroot>\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib/{libstd-*,libpanic_unwind-*,libcfg_if-*,libwindows_targets-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace_alloc-*,libunwind-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "kernel32.lib" "kernel32.lib" "kernel32.lib" "ntdll.lib" "userenv.lib" "ws2_32.lib" "dbghelp.lib" "/defaultlib:msvcrt" "/NXCOMPAT" "/OUT:D:\\git-repos\\github\\minimal-repros\\target\\tests\\macrotest\\debug\\build\\proc-macro2-5a2977cd70030fd1\\build_script_build-5a2977cd70030fd1.exe" "/OPT:REF,NOICF" "/DEBUG" "/PDBALTPATH:%_PDB%" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\liballoc.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libstd.natvis"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: LINK : fatal error LNK1104: cannot open file 'D:\git-repos\github\minimal-repros\target\tests\macrotest\debug\build\proc-macro2-5a2977cd70030fd1\build_script_build-5a2977cd70030fd1.exe'
error: could not compile `proc-macro2` (build script) due to 1 previous error
test tests::testb ... FAILED
test tests::testa ... FAILED

failures:

---- tests::testb stdout ----
Running 1 macro expansion tests
Expansion error:
   Compiling proc-macro2 v1.0.103
   Compiling quote v1.0.42
error: could not write output to D:\git-repos\github\minimal-repros\target\tests\macrotest\debug\build\proc-macro2-5a2977cd70030fd1\build_script_build-5a2977cd70030fd1.build_script_build.c80a3eaaf664fd79-cgu.0.rcgu.o: permission denied
error: could not compile `proc-macro2` (build script) due to 1 previous error
warning: build failed, waiting for other jobs to finish...





thread 'tests::testb' (54532) panicked at D:\.tools\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\macrotest-1.2.0\src\expand.rs:174:9:
1 of 1 tests failed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- tests::testa stdout ----
Running 1 macro expansion tests
Expansion error:
   Compiling proc-macro2 v1.0.103
   Compiling quote v1.0.42
error: linking with `link.exe` failed: exit code: 1104
  |
  = note: "C:\\Program Files\\Microsoft Visual Studio\\2022\\Enterprise\\VC\\Tools\\MSVC\\14.44.35207\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "C:\\Users\\MELVIN~1\\AppData\\Local\\Temp\\rustcJLej2A\\symbols.o" "<2 object files omitted>" "<sysroot>\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib/{libstd-*,libpanic_unwind-*,libcfg_if-*,libwindows_targets-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace_alloc-*,libunwind-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "kernel32.lib" "kernel32.lib" "kernel32.lib" "ntdll.lib" "userenv.lib" "ws2_32.lib" "dbghelp.lib" "/defaultlib:msvcrt" "/NXCOMPAT" "/OUT:D:\\git-repos\\github\\minimal-repros\\target\\tests\\macrotest\\debug\\build\\quote-4e19cce3fccb8eb3\\build_script_build-4e19cce3fccb8eb3.exe" "/OPT:REF,NOICF" "/DEBUG" "/PDBALTPATH:%_PDB%" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\liballoc.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libstd.natvis"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: LINK : fatal error LNK1104: cannot open file 'D:\git-repos\github\minimal-repros\target\tests\macrotest\debug\build\quote-4e19cce3fccb8eb3\build_script_build-4e19cce3fccb8eb3.build_script_build.b4eabd596b6c2fca-cgu.0.rcgu.o'
error: could not compile `quote` (build script) due to 1 previous error
warning: build failed, waiting for other jobs to finish...





thread 'tests::testa' (72240) panicked at D:\.tools\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\macrotest-1.2.0\src\expand.rs:174:9:
1 of 1 tests failed


failures:
    tests::testa
    tests::testb

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.11s
```