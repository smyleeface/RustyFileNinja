use std::fs::{self};
use std::path::Path;
use std::io::{self, stdin, stdout, Write};


/// Begins the delete file process.
pub fn run() -> io::Result<()> {
    let inputs = prompt_input()?;
    delete_file(inputs)?;
    Ok(())
}

/// Returns the input provided by the user.
fn prompt_input() -> io::Result<(String)> {

    let mut name_of_file = String::new();
    while name_of_file.is_empty() {
        print!("\nName of file: ");
        stdout().flush()?;
        stdin().read_line(&mut name_of_file)?;
        name_of_file = name_of_file.trim().to_string();
    }

    Ok(name_of_file)
}

/// Deletes a file given the filename
pub fn delete_file(name_of_file: String) -> io::Result<()> {
    let file_path = Path::new(&name_of_file);
    fs::remove_file(file_path)?;
    println!("\nFile deleted {}", name_of_file);
    Ok(())
}
