use std::fs::File;
use std::{fs, io};
use std::io::{Read, Write};
use std::path::Path;

/// Read the file contents
pub(crate) fn get_contents(file_to_read: String) -> String {
    let mut file_data= String::new();
    let mut file_client = File::open(file_to_read).unwrap();
    file_client.read_to_string(&mut file_data).unwrap();
    file_data
}

/// Create a file given a name of file and content
pub(crate) fn create_file(name_of_file: String, content: String) -> io::Result<()> {
    let file_path = Path::new(&name_of_file);
    let mut output_file = File::create(file_path)?;
    write!(output_file, "{}", content)?;
    Ok(())
}

/// Remove a file given a name

pub(crate) fn remove_file(name_of_file: String) -> io::Result<()> {
    let file_path = Path::new(&name_of_file);
    fs::remove_file(file_path)
}

pub(crate) fn file_exists(name_of_file: String) -> bool {
    let file_path = Path::new(&name_of_file);
    Path::exists(file_path)
}