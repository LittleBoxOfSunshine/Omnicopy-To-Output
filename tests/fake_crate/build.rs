use build_copy_to_output::copy_to_output;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bin-place resource files
    copy_to_output("schemas").unwrap();
    copy_to_output("config.toml").unwrap();

    Ok(())
}
