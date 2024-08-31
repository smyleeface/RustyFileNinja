use std::io::{self};
use crate::utils;

/// Begins the create file process.
pub fn run(mut name_of_file: String, mut content: String) -> io::Result<()> {
    name_of_file = prompt_input(name_of_file, String::from("\nName of file: "), true);
    content = prompt_input(content, String::from("Content to write to the file: \n"), false);
    create_file(name_of_file, content)?;
    Ok(())
}

/// Returns the input provided by the user.
fn prompt_input(value: String, mut prompt: String, required: bool) -> String {
    if !value.is_empty() {
        return value
    }
    if required {
        utils::prompt_for_required_value(String::from(prompt))
    } else {
        utils::prompt_for_value(String::from(prompt))
    }
}

/// Creates a file given the filename and content
pub(crate) fn create_file(name_of_file: String, content: String) -> io::Result<()> {
    utils::file_io::create_file(name_of_file.clone(), content)?;
    println!("\nFile created {}", name_of_file);
    Ok(())
}
