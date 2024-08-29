use std::fs::{File};
use std::io::{self, stdin, stdout, Write, BufRead, Read};
use std::ops::Add;
use std::path::Path;

/// Begins the combine file process.
pub fn run() -> io::Result<()> {
    let inputs = prompt_input()?;
    combine_files(inputs.0, inputs.1, inputs.2)?;
    Ok(())
}

fn get_file_contents(file_to_read: String) -> String {
    let mut file_data= String::new();
    let mut file_client = File::open(file_to_read).unwrap();
    file_client.read_to_string(&mut file_data).unwrap();
    file_data
}

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

/// Returns the input provided by the user.
fn prompt_input() -> io::Result<(String, String, String)> {
    let source_file_1 = prompt_for_required_value(String::from("\nFirst File: "));
    let source_file_2 = prompt_for_required_value(String::from("\nSecond File: "));
    let destination_file = prompt_for_required_value(String::from("\nFile destination: "));
    Ok((source_file_1, source_file_2, destination_file))
}

/// Creates a new file with the contents of two provided files
pub fn combine_files(source_1: String, source_2: String, destination: String) -> io::Result<()> {
    let mut new_string = String::new();
    let source_data_1 = get_file_contents(source_1);
    let source_data_2= get_file_contents(source_2);
    new_string.push_str(source_data_1.as_str());
    new_string.push_str(source_data_2.as_str());
    let destination_path = Path::new(&destination);
    let mut destination_file = File::create(destination_path)?;
    write!(destination_file, "{}", new_string)?;
    println!("\nFile created {}", destination);
    Ok(())
}
