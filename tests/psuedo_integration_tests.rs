
// These aren't actually integration tests, the point is to verify that target directory when
// an integration test is run is verified to work.

#[test]
fn file_copy() {
    assert_eq!(5, 5);
}

#[test]
fn directory_copy() {
    assert_eq!(5, 5);
}

#[test]
fn directory_copy_recuses() {
    assert_eq!(5, 5);
}
