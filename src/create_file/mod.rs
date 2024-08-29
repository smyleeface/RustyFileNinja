use std::fs::{self, File};
use std::path::Path;
use std::io::{self, stdin, stdout, Write};

use crate::utils;

/// Begins the create file process.
pub fn run() -> io::Result<()> {
    let inputs = prompt_input()?;
    create_file(inputs.0, inputs.1)?;
    Ok(())
}

/// Returns the input provided by the user.
fn prompt_input() -> io::Result<(String, String)> {
    let name_of_file = utils::prompt_for_required_value(String::from("\nName of file: "));
    let content = utils::prompt_for_required_value(String::from("Content to write to the file: \n"));
    Ok((name_of_file, content))
}

/// Creates a file given the filename and content
pub fn create_file(name_of_file: String, content: String) -> io::Result<()> {
    let file_path = Path::new(&name_of_file);
    let mut output_file = File::create(file_path)?;
    write!(output_file, "{}", content)?;
    println!("\nFile created {}", name_of_file);
    Ok(())
}
