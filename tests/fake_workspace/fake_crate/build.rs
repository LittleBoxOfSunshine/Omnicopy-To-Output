use omnicopy_to_output::copy_to_output;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("TEST: {:?}", std::env::current_dir().unwrap());
    println!("RES: {:?}", std::path::Path::new("./res").exists());
    println!("RES: {:?}", std::path::Path::new("./res").join("empty").exists());

    // Bin-place resource files
    copy_to_output("res/nested").unwrap();
    copy_to_output("res/empty").unwrap();
    copy_to_output("res/test.dat").unwrap();
    copy_to_output("res/test.txt").unwrap();
    copy_to_output("res/second.txt").unwrap();

    Ok(())
}
