use omnicopy_to_output::{copy_to_output, copy_to_output_for_build_type};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Can't commit empty directories to git, dynamically create to ensure copying them works properly.
    fs::create_dir_all(Path::new("res").join("empty")).unwrap();
    fs::create_dir_all(Path::new("res").join("nested").join("emptier")).unwrap();
    fs::create_dir_all(
        Path::new("res")
            .join("nested")
            .join("doublenested")
            .join("emptiest"),
    )
    .unwrap();

    // Bin-place resource files
    copy_to_output("res/nested").unwrap();
    copy_to_output("res/empty").unwrap();
    copy_to_output("res/test.dat").unwrap();
    copy_to_output("res/test.txt").unwrap();
    copy_to_output("res/second.txt").unwrap();

    Ok(())
}
