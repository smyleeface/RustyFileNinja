use std::fs::{self, File};
use std::path::Path;
use std::io::{self, stdin, stdout, Write};


/// Begins the create file process.
pub fn run() -> io::Result<()> {
    let inputs = prompt_input()?;
    create_file(inputs.0, inputs.1)?;
    Ok(())
}

/// Returns the input provided by the user.
fn prompt_input() -> io::Result<(String, String)> {

    let mut name_of_file = String::new();
    while name_of_file.is_empty() {
        print!("\nName of file: ");
        stdout().flush()?;
        stdin().read_line(&mut name_of_file)?;
        name_of_file = name_of_file.trim().to_string();
    }

    let mut content: String = String::new();
    println!("Content to write to the file: ");
    stdout().flush()?;
    stdin().read_line(&mut content)?;
    Ok((name_of_file, content))
}

/// Creates a file given the filename and content
pub fn create_file(name_of_file: String, content: String) -> io::Result<()> {
    let file_path = Path::new(&name_of_file);
    let mut output_file = File::create(file_path)?;
    write!(output_file, "{}", content)?;
    println!("\nFile created {}", name_of_file);
    Ok(())
}
