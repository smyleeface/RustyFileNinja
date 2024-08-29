pub(crate) mod file_io;

use std::io::{stdin, stdout, Write};

/// Returns an input provided by the user. Continues to prompt until a user enter a value.
pub(crate) fn prompt_for_required_value(message_prompt: String) -> String {
    let mut source_file = String::new();
    while source_file.is_empty() {
        print!("{}", message_prompt);
        stdout().flush().unwrap();
        stdin().read_line(&mut source_file).unwrap();
        source_file = source_file.trim().to_string();
    }
    source_file
}

/// Returns an input provided by the user. Empty value is allowed.
pub(crate) fn prompt_for_value(message_prompt: String) -> String {
    let mut content: String = String::new();
    print!("{}", message_prompt);
    stdout().flush().unwrap();
    stdin().read_line(&mut content).unwrap();
    content
}