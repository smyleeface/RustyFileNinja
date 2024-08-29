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
    let file_path = Path::new(&name_of_file);
    fs::remove_file(file_path)?;
    println!("\nFile deleted {}", name_of_file);
    Ok(())
}
