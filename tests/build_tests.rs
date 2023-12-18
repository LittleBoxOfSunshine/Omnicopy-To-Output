use crate::common::{build_environment, build_environment_with_target, custom_test_target, fake_crate_in_tempdir, fake_workspace_in_tempdir, validate};

mod common;

#[test]
fn build_crate() {
    let environment = fake_crate_in_tempdir();
    build_environment(&environment);
    validate(&environment, None);
}

#[test]
fn build_crate_target_specified() {
    let environment = fake_crate_in_tempdir();
    build_environment_with_target(&environment, custom_test_target());
    validate(&environment, Some(custom_test_target()));
}

#[test]
fn build_workspace() {
    let environment = fake_workspace_in_tempdir();
    build_environment(&environment);
    validate(&environment, None);
}

#[test]
fn build_workspace_target_specified() {
    let environment = fake_workspace_in_tempdir();
    build_environment_with_target(&environment, custom_test_target());
    validate(&environment, Some(custom_test_target()));
}
