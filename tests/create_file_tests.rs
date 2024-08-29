use std::fs;
use std::fs::File;
use std::io::{self, BufRead};

use fini::create_file;

#[cfg(test)]
mod tests {
    use std::io::Read;
    use super::*;

    #[test]
    fn test_create_file() {
        // Arrange
        let mut file_data = String::new();
        create_file::create_file(String::from("foo"), String::from("bar")).unwrap();
        let output_file = File::open("foo");

        // Act
        output_file.unwrap().read_to_string(&mut file_data).unwrap();

        //Assert
        assert_eq!(file_data, "bar");

        // Teardown
        fs::remove_file("foo").unwrap();
    }
}