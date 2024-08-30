mod commands;

use std::io::{self};
use std::str::FromStr;
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut command= "help";
    if args.len() > 1 {
        command = args.get(1).unwrap();
    }
    let command_enum = commands::Commands::from_str(command).unwrap();
    command_enum.run();
    Ok(())
}
