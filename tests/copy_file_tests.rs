use std::fs::{File};
use std::io::{Write};

use fini::copy_file;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io;
    use std::io::Read;
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
        let content = String::from("bar content");
        let source_filename = String::from("foo_file_to_copy");
        let destination_filename = String::from("foo_file_copy");
        set_up(source_filename.clone(), content.clone()).unwrap();

        // Act
        copy_file::copy_file(source_filename.clone(), destination_filename.clone()).unwrap();

        //Assert
        let mut result_data = String::new();
        let result_file = File::open(destination_filename.clone());
        result_file.unwrap().read_to_string(&mut result_data).unwrap();
        assert_eq!(result_data, content);

        // Teardown
        fs::remove_file(source_filename).unwrap();
        fs::remove_file(destination_filename).unwrap();
    }
}