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

pub fn prompt_to_overwrite(name_of_file: String) -> bool {
    if file_io::file_exists(name_of_file.clone()) {
        let mut overwrite_ok = String::new();
        overwrite_ok = prompt_input(overwrite_ok, format!("\nFile '{}' exists. Overwrite (Y/y)?  ", name_of_file), true);
        if overwrite_ok != "Y" && overwrite_ok != "y" {
            return false;
        }
    }
    true
}

pub fn prompt_for_new_location(mut name_of_file: String, prompt: String) -> String {
    let mut does_file_exist = file_io::file_exists(name_of_file.clone());
    while !does_file_exist {
        let mut another_source_value = String::new();
        println!("\nFile doesn't exist {}", name_of_file.clone());
        another_source_value = prompt_input(another_source_value, prompt.clone(), true);
        does_file_exist = file_io::file_exists(another_source_value.clone());
        name_of_file = another_source_value;
    }
    name_of_file
}
