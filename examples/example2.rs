use permissions::{is_file_executable, is_file_readable, is_file_removable, is_file_writable};

use std::{
    env,
    fs::{self, File},
    io,
};

fn main() -> io::Result<()> {
    let this_program_path = env::args().next().unwrap();
    assert!(is_file_executable(&this_program_path)?);

    let this_file = "examples/example2.rs";
    assert!(is_file_readable(&this_file)?);

    let temp_file = "temp.txt";
    File::create(temp_file)?;
    assert!(is_file_writable(&temp_file)?);
    assert!(is_file_removable(&temp_file)?);
    fs::remove_file(temp_file)?;

    // ---

    // Let's try with directories too
    let temp_directory = "temp/";
    fs::create_dir(temp_directory)?;
    assert!(is_file_readable(&this_file_parent_dir)?);
    assert!(is_file_writable(&temp_directory)?);
    assert!(is_file_executable(&temp_directory)?);
    assert!(is_file_removable(&temp_directory)?);
    fs::remove_dir(temp_directory)?;

    println!("Finished.");
    Ok(())
}
