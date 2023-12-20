use crate::common::{
    build_environment, fake_crate_in_tempdir, get_target_path, validate, TestEnvironment,
};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::time::SystemTime;

pub mod common;

// Verify cache clears occur by:
//  1. Clone to temp dir and build
//  2. Edit a resource file
//  3. Build again
//  4. Verify the output file has updated

#[test]
fn file_updated() {
    let environment = create_and_build();
    let test_file = environment.path.join("res").join("test.txt");

    edit_build_validate_updated(environment, test_file);
}

fn edit_build_validate_updated(environment: TestEnvironment, test_file: PathBuf) {
    let target_file = get_target_path(&environment, None);
    let original_modified = get_last_modified(&target_file);

    edit_file(&test_file);
    build_environment(&environment);
    validate_updated(target_file, original_modified)
}

fn create_and_build() -> TestEnvironment {
    let environment = fake_crate_in_tempdir();
    build_environment(&environment);
    validate(&environment, None);

    environment
}

fn edit_file(path: &PathBuf) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    file.write_all("This causes cache invalidation".as_ref())
        .unwrap();
}

#[test]
fn directory_updated() {
    let environment = create_and_build();
    let test_file = environment
        .path
        .join("res")
        .join("nested")
        .join("test2.txt");

    edit_build_validate_updated(environment, test_file);
}

#[test]
fn directory_updated_recuses() {
    let environment = create_and_build();
    let test_file = environment
        .path
        .join("res")
        .join("nested")
        .join("doublenested")
        .join("test3.txt");

    edit_build_validate_updated(environment, test_file);
}

fn validate_updated(path: PathBuf, original_modified: SystemTime) {
    assert!(fs::metadata(path).unwrap().modified().unwrap() > original_modified)
}

fn get_last_modified(path: &PathBuf) -> SystemTime {
    fs::metadata(path).unwrap().modified().unwrap()
}
