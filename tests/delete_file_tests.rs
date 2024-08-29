use std::fs::{File};
use std::io::{Write};

use fini::delete_file;

#[cfg(test)]
mod tests {
    use std::io;
    use std::path::Path;
    use super::*;

    // Create a file for the copy command
    fn set_up(filename: String, content: String) -> io::Result<()> {
        let file_path = Path::new(filename.as_str());
        let mut output_file = File::create(file_path)?;
        write!(output_file, "{}", content)
    }

    #[test]
    fn test_copy_file() {
        // Arrange
        let filename = String::from("foo_file_to_delete");
        set_up(filename.clone(), "abc".to_string()).unwrap();
        // let's make sure the file exists to confirm it's really gone
        assert!(Path::new(&filename.clone()).exists());

        // Act
        delete_file::delete_file(filename.clone()).expect("error running test test_copy_file");

        //Assert
        assert!(!Path::new(&filename.clone()).exists());
    }
}