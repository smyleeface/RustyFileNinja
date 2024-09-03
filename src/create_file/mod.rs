use std::io::{self};
use std::process::exit;
use crate::utils;

/// Begins the create file process.
pub fn run(mut name_of_file: String, mut content: String) -> io::Result<()> {
    name_of_file = utils::prompt_input(name_of_file, String::from("\nName of file: "), true);
    if !utils::prompt_to_overwrite(name_of_file.clone()) {
        exit(0)
    }
    content = utils::prompt_input(content, String::from("Content to write to the file: \n"), false);
    create_file(name_of_file, content)?;
    Ok(())
}

/// Creates a file given the filename and content
pub(crate) fn create_file(name_of_file: String, content: String) -> io::Result<()> {
    utils::file_io::create_file(name_of_file.clone(), content)?;
    println!("\nFile created {}", name_of_file);
    Ok(())
}
