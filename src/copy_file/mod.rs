use std::io::{self};
use std::process::exit;
use crate::utils;

/// Begins the copy file process.
pub fn run(mut source_file: String, mut destination_location: String) -> io::Result<()> {
    source_file = utils::prompt_input(source_file, String::from("\nFile source: "), true);
    source_file = utils::prompt_for_new_location(source_file, String::from("\nFile source: "));
    destination_location = utils::prompt_input(destination_location, String::from("\nFile destination: "), true);
    if !utils::prompt_to_overwrite(destination_location.clone()) {
        exit(0)
    }
    copy_file(source_file, destination_location)?;
    Ok(())
}

/// Copies a file to another location when given both the source and destination.
pub fn copy_file(source_location: String, destination_location: String) -> io::Result<()> {
    let source_content = utils::file_io::get_contents(source_location.clone());
    utils::file_io::create_file(destination_location.clone(), source_content)?;
    println!("\nFile created {}", destination_location);
    Ok(())
}
