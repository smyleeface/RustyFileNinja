use std::io::{self, stdin, stdout, Write};


/// Begins the copy file process.
pub fn run() -> io::Result<()> {
    let inputs = prompt_input()?;
    println!("{:?}", inputs);
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
