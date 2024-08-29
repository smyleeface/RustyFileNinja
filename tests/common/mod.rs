use std::fs::{File};
use std::io::{self, Read, Write};
use std::path::Path;

pub fn setup(filename: String, content: String) -> io::Result<()> {
    let file_path = Path::new(filename.as_str());
    let mut output_file = File::create(file_path)?;
    write!(output_file, "{}", content)
}

pub fn read_file(file_to_read: String) -> String {
    let mut file_data= String::new();
    let mut file_client = File::open(file_to_read).unwrap();
    file_client.read_to_string(&mut file_data).unwrap();
    file_data
}