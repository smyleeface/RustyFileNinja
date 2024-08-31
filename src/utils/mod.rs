pub(crate) mod file_io;

use std::io::{stdin, stdout, Write};
use crate::utils;

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

/// Returns the input provided by the user.
pub(crate) fn prompt_input(value: String, prompt: String, required: bool) -> String {
    if !value.is_empty() {
        return value
    }
    if required {
        prompt_for_required_value(String::from(prompt))
    } else {
        prompt_for_value(String::from(prompt))
    }
}