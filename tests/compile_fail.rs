use std::fs;
use std::path::Path;
use std::process::Command;

/// Build a throwaway crate whose `main.rs` is `source` and that depends on
/// this crate via a path dependency.  Returns `true` when the build *fails*
/// (i.e. the code correctly refuses to compile).
fn build_fails(name: &str, source: &str, features: &[&str]) -> bool {
    let tmp = std::env::temp_dir().join(format!("foo_compile_fail_{name}"));
    let src_dir = tmp.join("src");
    fs::create_dir_all(&src_dir).unwrap();

    let manifest_dir = env!("CARGO_MANIFEST_DIR").replace('\\', "/");
    let feature_list = features
        .iter()
        .map(|f| format!("\"{f}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let manifest = format!(
        "[package]\nname = \"cf-{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n\
         [dependencies]\nfoo = {{ path = \"{manifest_dir}\", features = [{feature_list}] }}\n"
    );
    fs::write(tmp.join("Cargo.toml"), manifest).unwrap();
    fs::write(src_dir.join("main.rs"), source).unwrap();

    let output = Command::new("cargo")
        .arg("build")
        .current_dir(&tmp)
        .output()
        .expect("failed to run cargo");

    let _ = fs::remove_dir_all(&tmp);
    !output.status.success()
}

fn run_compile_fail_dir(dir: &Path, features: &[&str]) -> usize {
    let mut tested = 0;
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|e| e == "rs") {
            let name = path.file_stem().unwrap().to_str().unwrap();
            let source = fs::read_to_string(&path).unwrap();
            assert!(
                build_fails(name, &source, features),
                "{name} should fail to compile but succeeded",
            );
            tested += 1;
        }
    }
    tested
}

#[test]
fn zst_compile_failures() {
    let tested = run_compile_fail_dir(Path::new("tests/compile_fail"), &[]);
    assert!(tested > 0, "no compile-fail test files found");
}

#[test]
#[cfg(feature = "nightly")]
fn zst_compile_failures_const_generic() {
    let tested = run_compile_fail_dir(
        Path::new("tests/compile_fail_nightly"),
        &["nightly"],
    );
    assert!(tested > 0, "no nightly compile-fail test files found");
}
