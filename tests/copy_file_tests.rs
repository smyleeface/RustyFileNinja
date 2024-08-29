mod common;

use fini::copy_file;

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;


    #[test]
    fn test_copy_file() {
        // Arrange
        let content = String::from("bar content");
        let source_filename = String::from("foo_file_to_copy");
        let destination_filename = String::from("foo_file_copy");
        common::setup(source_filename.clone(), content.clone()).unwrap();

        // Act
        copy_file::copy_file(source_filename.clone(), destination_filename.clone()).unwrap();

        //Assert
        let result_data = common::read_file(destination_filename.clone());
        assert_eq!(result_data, content);

        // Teardown
        fs::remove_file(source_filename).unwrap();
        fs::remove_file(destination_filename).unwrap();
    }
}