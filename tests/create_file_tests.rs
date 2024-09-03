mod common;

use fini::create_file;



#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;


    #[test]
    fn test_proceed_to_write_file() {
        // Arrange
        let name_of_file = String::from("bar");
        common::setup(name_of_file.clone(), String::new()).unwrap();

        // Act
        let result_data = create_file::proceed_to_write_file(name_of_file.clone());

        //Assert
        assert_eq!(result_data, false);

        // Arrange
        fs::remove_file(name_of_file.clone()).unwrap();

        // Act
        let result_data = create_file::proceed_to_write_file(name_of_file.clone());

        //Assert
        assert_eq!(result_data, true);
    }
}