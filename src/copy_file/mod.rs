use std::fs::File;
use std::io::{self, stdin, stdout, Read, Write};
use std::path::Path;

/// Begins the copy file process.
pub fn run() -> io::Result<()> {
    let inputs = prompt_input()?;
    copy_file(inputs.0, inputs.1)?;
    Ok(())
}

/// Returns an input provided by the user.
fn prompt_for_required_value(message_prompt: String) -> String {
    let mut source_file = String::new();
    while source_file.is_empty() {
        print!("{}", message_prompt);
        stdout().flush().unwrap();
        stdin().read_line(&mut source_file).unwrap();
        source_file = source_file.trim().to_string();
    }
    source_file
}

/// Returns all the input provided by the user.
fn prompt_input() -> io::Result<(String, String)> {
    let source_file = prompt_for_required_value(String::from("\nFile source: "));
    let destination_file = prompt_for_required_value(String::from("\nFile destination: "));
    Ok((source_file, destination_file))
}

/// Copies a file to another location when given both the source and destination.
pub fn copy_file(source_location: String, destination_location: String) -> io::Result<()> {
    let mut source_content = String::new();
    let source_path = Path::new(&source_location);
    let mut source_file = File::open(source_path)?;
    source_file.read_to_string(&mut source_content)?;
    let mut destination_file = File::create(destination_location.clone())?;
    write!(destination_file, "{}", source_content)?;
    println!("\nFile created {}", destination_location);
    Ok(())
}
