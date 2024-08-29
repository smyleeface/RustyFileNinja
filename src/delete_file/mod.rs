use std::fs::{self};
use std::path::Path;
use std::io::{self, stdin, stdout, Write};
use crate::utils;

/// Begins the delete file process.
pub fn run() -> io::Result<()> {
    let inputs = prompt_input()?;
    delete_file(inputs)?;
    Ok(())
}

/// Returns the input provided by the user.
fn prompt_input() -> io::Result<(String)> {
    let name_of_file = utils::prompt_for_required_value(String::from("\nName of file: "));
    Ok(name_of_file)
}

/// Deletes a file given the filename
pub fn delete_file(name_of_file: String) -> io::Result<()> {
    utils::file_io::remove_file(name_of_file.clone())?;
    println!("\nFile deleted {}", name_of_file);
    Ok(())
}
