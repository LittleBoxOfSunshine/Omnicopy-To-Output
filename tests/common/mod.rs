use std::fs;
use std::path::{Path, PathBuf};
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use tempfile::TempDir;

pub struct TestEnvironment {
    _handle: TempDir,
    pub path: PathBuf
}

// Creates a copy of the fake crate in a temp dir and returns handle
pub fn fake_crate_in_tempdir() -> TestEnvironment {
    let mut environment = fake_workspace_in_tempdir();

    // In order for the fake crate to work as expected, kill the workspace file that owns it.
    fs::remove_file(environment.path.join("../../Cargo.toml")).expect("Failed to delete workspace toml");

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

    let path = dir.path().join("../fake_workspace");

    TestEnvironment {
        _handle: dir,
        path
    }
}

pub fn build_environment(environment: &TestEnvironment) {
    std::process::Command::new("cargo")
        .current_dir(environment.path.to_str().expect("Couldn't to_string environment path"))
        .arg("build")
        .output().expect("failed to execute process");
}

pub fn build_environment_with_target(environment: &TestEnvironment, target: String) {
    std::process::Command::new("cargo")
        .current_dir(environment.path.to_str().expect("Couldn't to_string environment path"))
        .arg("build")
        .arg(format!("--target {target}"))
        .output().expect("failed to execute process");
}

pub fn validate(environment: &TestEnvironment) {
    assert!(environment.path.join("empty").exists());
    assert!(environment.path.join("nested").exists());

    assert!(environment.path.join("nested/doublenested").exists());
    assert!(environment.path.join("nested/doublenested/emptiest").exists());
    assert!(environment.path.join("nested/doublenested/seconddoublenested.txt").exists());
    assert!(environment.path.join("nested/doublenested/test3.txt").exists());

    assert!(environment.path.join("nested/emptier").exists());
    assert!(environment.path.join("nested/secondnested.txt").exists());
    assert!(environment.path.join("nested/test2.txt").exists());

    assert!(environment.path.join("second.txt").exists());
    assert!(environment.path.join("test.dat").exists());
    assert!(environment.path.join("test.txt").exists());
}

#[cfg(target_os = "linux")]
pub fn custom_test_target() -> String {
    "x86_64-unknown-linux-gnu".to_string()
}

// Just makes running the tests on a windows machine easier
#[cfg(target_os = "windows")]
pub fn custom_test_target() -> String {
    "x86_64-pc-windows-msvc".to_string()
}
