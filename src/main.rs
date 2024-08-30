mod commands;

use std::io::{self};
use std::str::FromStr;
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = &args.get(1).expect("Command not provided");
    let command_enum = commands::Commands::from_str(command.as_str()).unwrap();
    command_enum.run();
    Ok(())
}
