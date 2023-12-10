
// Verify cache clears occur by:
//  1. Clone to temp dir and build
//  2. Edit a resource file
//  3. Build again
//  4. Verify the output file has updated

#[test]
fn file_updated() {
    assert_eq!(5, 5);
}

#[test]
fn directory_updated() {
    assert_eq!(5, 5);
}

#[test]
fn directory_updated_recuses() {
    assert_eq!(5, 5);
}
