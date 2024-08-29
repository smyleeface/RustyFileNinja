mod common;

use std::fs::{File};
use std::io::{Write};

use fini::combine_files;

#[cfg(test)]
mod tests {
    use std::{fs, io};
    use std::io::Read;
    use std::path::Path;
    use super::*;

    #[test]
    fn test_combine_files() {
        // Arrange
        let filename_1 = String::from("foo_file_1");
        common::setup(filename_1.clone(), "abc".to_string()).unwrap();
        let filename_2 = String::from("foo_file_2");
        common::setup(filename_2.clone(), "123".to_string()).unwrap();
        let destination = String::from("combined_foo_file");

        // Act
        combine_files::combine_files(filename_1.clone(), filename_2.clone(), destination.clone()).expect("error running test test_combine_files");

        //Assert
        let mut result_content = String::new();
        let result_file = File::open(destination.clone());
        result_file.unwrap().read_to_string(&mut result_content).unwrap();
        assert_eq!(result_content, "abc123");

        // Teardown
        fs::remove_file("foo_file_1").unwrap();
        fs::remove_file("foo_file_2").unwrap();
        fs::remove_file("combined_foo_file").unwrap();
    }
}