use std::io::{self};
use crate::utils;

/// Begins the combine file process.
pub fn run() -> io::Result<()> {
    let inputs = prompt_input()?;
    combine_files(inputs.0, inputs.1, inputs.2)?;
    Ok(())
}

/// Returns the input provided by the user.
fn prompt_input() -> io::Result<(String, String, String)> {
    let source_file_1 = utils::prompt_for_required_value(String::from("\nFirst File: "));
    let source_file_2 = utils::prompt_for_required_value(String::from("\nSecond File: "));
    let destination_file = utils::prompt_for_required_value(String::from("\nFile destination: "));
    Ok((source_file_1, source_file_2, destination_file))
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
