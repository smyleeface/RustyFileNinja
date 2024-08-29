use std::fs::{File};
use std::io::{self, Read, Write};
use std::path::Path;

pub fn setup(filename: String, content: String) -> io::Result<()> {
    let file_path = Path::new(filename.as_str());
    let mut output_file = File::create(file_path)?;
    write!(output_file, "{}", content)
}
