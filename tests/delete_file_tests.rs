mod common;

use std::io::{Write};

use fini::delete_file;

#[cfg(test)]
mod tests {
    use std::io;
    use std::path::Path;
    use super::*;


    #[test]
    fn test_copy_file() {
        // Arrange
        let filename = String::from("foo_file_to_delete");
        common::setup(filename.clone(), "abc".to_string()).unwrap();
        // let's make sure the file exists to confirm it's really gone
        assert!(Path::new(&filename.clone()).exists());

        // Act
        delete_file::delete_file(filename.clone()).expect("error running test test_copy_file");

        //Assert
        assert!(!Path::new(&filename.clone()).exists());
    }
}