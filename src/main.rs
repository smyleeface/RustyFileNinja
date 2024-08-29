use std::io::{self};
use std::str::FromStr;
use std::env;

use fini::{create_file, copy_file, delete_file};

#[derive(Debug)]
pub enum Commands {
    Create,
    Copy,
    Combine,
    Delete
}

impl FromStr for Commands {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "create" => Ok(Commands::Create),
            "copy" => Ok(Commands::Copy),
            "combine" => Ok(Commands::Combine),
            "delete" => Ok(Commands::Delete),
            _ => panic!("Command not supported")
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = &args.get(1).expect("Command not provided");
    let command_enum = Commands::from_str(command.as_str()).unwrap();
    match command_enum {
        Commands::Create => {
            println!("Commands::Create");
            create_file::run().expect("Error running create_file command");
        },
        Commands::Copy => {
            println!("Commands::Copy");
            copy_file::run().expect("Error running copy_file command");
        },
        Commands::Combine => println!("Commands::Combine"),
        Commands::Delete => {
            println!("Commands::Delete");
            delete_file::run().expect("Error running delete_file command");
        }
    }
    Ok(())
}
