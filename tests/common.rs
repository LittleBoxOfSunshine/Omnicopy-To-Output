use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::TempDir;

pub struct TestEnvironment {
    _handle: TempDir,
    pub path: PathBuf,
    _out_handle: Option<TempDir>,
    pub out_path: Option<PathBuf>,
}

// Creates a copy of the fake crate in a temp dir and returns handle
pub fn fake_crate_in_tempdir(custom_out_dir: bool) -> TestEnvironment {
    let mut environment = fake_workspace_in_tempdir(custom_out_dir);

    // In order for the fake crate to work as expected, kill the workspace file that owns it.
    fs::remove_file(environment.path.join("Cargo.toml")).expect("Failed to delete workspace toml");

    // We just reused the workspace builder, fake crate is nested just append to the path.
    environment.path = environment.path.join("fake_crate");

    environment
}

pub fn fake_workspace_in_tempdir(custom_out_dir: bool) -> TestEnvironment {
    let dir = TempDir::new().expect("Failed to create temp dir");

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    copy_items(&[Path::new("./")], &dir.path(), &options).expect("Failed to copy");

    let path = dir.path().join("tests").join("fake_workspace");

    let mut environment = TestEnvironment {
        _handle: dir,
        path,
        _out_handle: None,
        out_path: None,
    };

    if custom_out_dir {
        let out_dir = TempDir::new().expect("Failed to create temp dir");
        environment.out_path = Some(PathBuf::from(out_dir.path()));
        environment._out_handle = Some(out_dir);
    }

    environment
}

pub fn build_environment(environment: &TestEnvironment) {
    let result = base_cargo_command(environment)
        .output()
        .expect("failed to execute process");

    dump_result(result);
}

fn base_cargo_command(environment: &TestEnvironment) -> Command {
    let mut command = Command::new("cargo");
    command
        .current_dir(
            environment
                .path
                .to_str()
                .expect("Couldn't to_string environment path"),
        )
        .arg("build");

    if let Some(path) = &environment.out_path {
        command.env("CARGO_TARGET_DIR", path);
    }

    command
}

fn dump_result(output: Output) {
    println!("{:?}", output.status);
    println!("{}", std::str::from_utf8(&output.stdout).unwrap());
    println!("{}", std::str::from_utf8(&output.stderr).unwrap());
}

pub fn build_environment_with_target(environment: &TestEnvironment, target: String) {
    let result = base_cargo_command(environment)
        .arg("--target")
        .arg(target)
        .output()
        .expect("failed to execute process");

    dump_result(result);
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
    let base_path = if let Some(path) = &environment.out_path {
        PathBuf::from(path)
    } else {
        environment.path.join("target")
    };

    let base_path = if let Some(target) = target {
        base_path.join(target)
    } else {
        base_path
    };

    base_path.join("debug")
}

// Just makes running the tests on a windows or apple silicon mac machine easier
pub fn custom_test_target() -> String {
    if cfg!(target_os = "windows") {
        "x86_64-pc-windows-msvc".to_string()
    } else if cfg!(target_os = "macos") {
        "aarch64-apple-darwin".to_string()
    } else {
        "x86_64-unknown-linux-gnu".to_string()
    }
}
