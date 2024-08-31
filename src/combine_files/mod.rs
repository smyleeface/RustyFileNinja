use std::io::{self};
use crate::utils;

/// Begins the combine file process.
pub fn run(source_file_1: String, source_file_2: String, destination_file: String) -> io::Result<()> {
    let source_file_1 = utils::prompt_input(source_file_1, String::from("\nFirst File: "), true);
    let source_file_2 = utils::prompt_input(source_file_2, String::from("\nSecond File: "), true);
    let destination_file = utils::prompt_input(destination_file, String::from("\nFile destination: "), true);
    combine_files(source_file_1, source_file_2, destination_file)?;
    Ok(())
}

/// Creates a new file with the contents of two provided files
pub fn combine_files(source_1: String, source_2: String, destination: String) -> io::Result<()> {
    let mut new_string = String::new();
    let source_data_1 = utils::file_io::get_contents(source_1);
    let source_data_2= utils::file_io::get_contents(source_2);
    new_string.push_str(source_data_1.as_str());
    new_string.push_str(source_data_2.as_str());
    utils::file_io::create_file(destination.clone(), new_string)?;
    println!("\nFile created {}", destination);
    Ok(())
}
