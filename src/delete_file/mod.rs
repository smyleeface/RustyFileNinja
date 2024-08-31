use std::io::{self};
use crate::utils;

/// Begins the delete file process.
pub fn run(name_of_file: String) -> io::Result<()> {
    let name_of_file = utils::prompt_input(name_of_file, String::from("\nName of file: "), true);
    delete_file(name_of_file)?;
    Ok(())
}

/// Deletes a file given the filename
pub fn delete_file(name_of_file: String) -> io::Result<()> {
    utils::file_io::remove_file(name_of_file.clone())?;
    println!("\nFile deleted {}", name_of_file);
    Ok(())
}
