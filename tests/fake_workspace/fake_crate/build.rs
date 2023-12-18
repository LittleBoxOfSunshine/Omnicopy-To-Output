use omnicopy_to_output::copy_to_output;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bin-place resource files
    copy_to_output("res/nested").unwrap();
    copy_to_output("res/empty").unwrap();
    copy_to_output("res/test.dat").unwrap();
    copy_to_output("res/test.txt").unwrap();
    copy_to_output("res/second.txt").unwrap();

    Ok(())
}
