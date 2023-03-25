// Show usage of is_* functions
use std::{
    fs::{self, File},
    io,
};

#[cfg(unix)]
use std::env;

#[cfg(unix)]
use permissions::is_executable;
use permissions::{is_readable, is_removable, is_writable};

fn main() -> io::Result<()> {
    // Asserts with files

    #[cfg(unix)]
    {
        let this_program_path = env::args().next().unwrap();
        assert!(is_executable(&this_program_path)?);
    }

    let this_file = "examples/example2.rs";
    assert!(is_readable(&this_file)?);

    let temp_file = "temp.txt";
    File::create(temp_file)?;
    assert!(is_writable(&temp_file)?);
    assert!(is_removable(&temp_file)?);
    fs::remove_file(temp_file)?;

    // -------------------------

    // Let's try with directories too
    let temp_directory = "temp/";
    fs::create_dir(temp_directory)?;
    assert!(is_readable(&temp_directory)?);
    assert!(is_writable(&temp_directory)?);
    #[cfg(unix)]
    assert!(is_executable(&temp_directory)?);
    assert!(is_removable(&temp_directory)?);
    fs::remove_dir(temp_directory)?;

    println!("Finished.");
    Ok(())
}
