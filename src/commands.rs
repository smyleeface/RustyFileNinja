use std::str::FromStr;

use fini::{create_file, copy_file, delete_file, combine_files};

#[derive(Debug)]
pub enum Commands {
    Create,
    Copy,
    Combine,
    Delete
}

impl Commands {
    pub fn run(&self) -> () {
        match self {
            Commands::Create => {
                println!("Commands::Create");
                create_file::run().expect("Error running create_file command");
            },
            Commands::Copy => {
                println!("Commands::Copy");
                copy_file::run().expect("Error running copy_file command");
            },
            Commands::Combine => {
                println!("Commands::Combine");
                combine_files::run().expect("Error running combine_files command");
            },
            Commands::Delete => {
                println!("Commands::Delete");
                delete_file::run().expect("Error running delete_file command");
            }
        }
    }
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