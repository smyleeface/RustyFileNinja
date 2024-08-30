use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use fini::{create_file, copy_file, delete_file, combine_files};

#[derive(Debug, EnumIter, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Commands {
    Create,
    Copy,
    Combine,
    Delete,
    Help,
    Version
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
            Commands::Help => {
                println!("Commands::Help");
                println!("Please enter one of the following commands: \n");
                for command in Commands::iter() {
                    println!("{}", command)
                }
            }
            Commands::Version => {
                let version: &str = env!("CARGO_PKG_VERSION");
                println!("v{}", version);
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
            "version" => Ok(Commands::Version),
            _ => Ok(Commands::Help)
        }
    }
}