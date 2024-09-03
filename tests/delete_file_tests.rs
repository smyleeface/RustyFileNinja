mod common;

use fini::delete_file;

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;


    #[test]
    fn test_delete_file_doesnt_exist() {
        // Arrange
        let filename = String::from("foo_file_to_delete");

        // Act
        delete_file::delete_file(filename.clone()).expect("error running test test_delete_file");

        //Assert
        assert!(!Path::new(&filename.clone()).exists());
    }
}