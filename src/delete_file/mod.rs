use std::io::{self};
use crate::utils;
use crate::utils::file_io;

/// Begins the delete file process.
pub fn run(name_of_file: String) -> io::Result<()> {
    let name_of_file = utils::prompt_input(name_of_file, String::from("\nName of file: "), true);
    delete_file(name_of_file)?;
    Ok(())
}

/// Deletes a file given the filename
pub fn delete_file(name_of_file: String) -> io::Result<()> {
    if file_io::file_exists(name_of_file.clone()) {
        let mut confirmation = String::new();
        confirmation = utils::prompt_input(confirmation, String::from("\nAre you sure you want to delete this file (Y/y)? "), true);
        if confirmation != "Y" && confirmation != "y" {
            return Ok(())
        }
        file_io::remove_file(name_of_file.clone())?;
        println!("\nFile deleted {}", name_of_file);

    } else {
        println!("\nFile does not exist {}", name_of_file);
    }
    Ok(())
}
