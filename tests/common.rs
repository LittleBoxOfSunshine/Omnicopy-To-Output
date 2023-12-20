use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

pub struct TestEnvironment {
    _handle: TempDir,
    pub path: PathBuf,
}

// Creates a copy of the fake crate in a temp dir and returns handle
pub fn fake_crate_in_tempdir() -> TestEnvironment {
    let mut environment = fake_workspace_in_tempdir();

    // In order for the fake crate to work as expected, kill the workspace file that owns it.
    fs::remove_file(environment.path.join("Cargo.toml")).expect("Failed to delete workspace toml");

    // We just reused the workspace builder, fake crate is nested just append to the path.
    environment.path = environment.path.join("fake_crate");

    environment
}

pub fn fake_workspace_in_tempdir() -> TestEnvironment {
    let dir = TempDir::new().expect("Failed to create temp dir");

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    copy_items(&[Path::new("./")], &dir.path(), &options).expect("Failed to copy");

    let path = dir.path().join("tests").join("fake_workspace");

    TestEnvironment { _handle: dir, path }
}

pub fn build_environment(environment: &TestEnvironment) {
    let result = std::process::Command::new("cargo")
        .current_dir(
            environment
                .path
                .to_str()
                .expect("Couldn't to_string environment path"),
        )
        .arg("build")
        .output()
        .expect("failed to execute process");

    println!("{:?}", result.status);
    println!("{:?}", std::str::from_utf8(&result.stdout).unwrap());
    println!("{:?}", std::str::from_utf8(&result.stderr).unwrap());
}

pub fn build_environment_with_target(environment: &TestEnvironment, target: String) {
    let result = std::process::Command::new("cargo")
        .current_dir(
            environment
                .path
                .to_str()
                .expect("Couldn't to_string environment path"),
        )
        .arg("build")
        .arg("--target")
        .arg(target)
        .output()
        .expect("failed to execute process");

    println!("{:?}", result.status);
    println!("{:?}", std::str::from_utf8(&result.stdout).unwrap());
    println!("{:?}", std::str::from_utf8(&result.stderr).unwrap());
}

pub fn validate(environment: &TestEnvironment, target: Option<String>) {
    let base_path = get_target_path(environment, target);

    assert!(base_path.join("empty").exists());
    assert!(base_path.join("nested").exists());

    assert!(base_path.join("nested").join("doublenested").exists());
    assert!(base_path
        .join("nested")
        .join("doublenested")
        .join("emptiest")
        .exists());
    assert!(base_path
        .join("nested")
        .join("doublenested")
        .join("seconddoublenested.txt")
        .exists());
    assert!(base_path
        .join("nested")
        .join("doublenested")
        .join("test3.txt")
        .exists());

    assert!(base_path.join("nested").join("emptier").exists());
    assert!(base_path.join("nested").join("secondnested.txt").exists());
    assert!(base_path.join("nested").join("test2.txt").exists());

    assert!(base_path.join("second.txt").exists());
    assert!(base_path.join("test.dat").exists());
    assert!(base_path.join("test.txt").exists());
}

pub fn get_target_path(environment: &TestEnvironment, target: Option<String>) -> PathBuf {
    let base_path = environment.path.join("target");

    let base_path = if let Some(target) = target {
        base_path.join(target)
    } else {
        base_path
    };

    base_path.join("debug")
}

// Just makes running the tests on a windows machine easier
pub fn custom_test_target() -> String {
    if cfg!(target_os = "windows") {
        "x86_64-pc-windows-msvc".to_string()
    } else {
        "x86_64-unknown-linux-gnu".to_string()
    }
}
