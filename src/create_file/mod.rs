use std::io::{self};
use std::process::exit;
use crate::utils;

/// Begins the create file process.
pub fn run(mut name_of_file: String, mut content: String) -> io::Result<()> {
    name_of_file = utils::prompt_input(name_of_file, String::from("\nName of file: "), true);
    if !proceed_to_write_file(name_of_file.clone()) {
        exit(0)
    }
    content = utils::prompt_input(content, String::from("Content to write to the file: \n"), false);
    create_file(name_of_file, content)?;
    Ok(())
}

/// Run checks before performing command
///
/// # Arguments
///
/// * `name_of_file` - The name of the file to do checks on
///
/// # Examples
///
/// ```
/// // Arrange
/// use fini::create_file::proceed_to_write_file;
/// let name_of_file = String::from("bar");
/// let result_data = proceed_to_write_file(name_of_file.clone());
/// assert_eq!(result_data, true);
/// ```
pub fn proceed_to_write_file(name_of_file: String) -> bool {
    if utils::file_io::file_exists(name_of_file) {
        let mut overwrite_ok = String::new();
        overwrite_ok = utils::prompt_input(overwrite_ok, String::from("\nFile exists. Overwrite (Y/y)?  "), true);
        if overwrite_ok != "Y" && overwrite_ok != "y" {
            return false;
        }
    }
    true
}

/// Creates a file given the filename and content
pub(crate) fn create_file(name_of_file: String, content: String) -> io::Result<()> {
    utils::file_io::create_file(name_of_file.clone(), content)?;
    println!("\nFile created {}", name_of_file);
    Ok(())
}
