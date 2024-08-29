use std::io::{self};
use std::str::FromStr;
use std::env;


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
    if args.len() == 1 {
        panic!("Command not provided");
    }
    let command = Commands::from_str(&args[1].as_str()).unwrap();
    match command {
        Commands::Create => println!("Commands::Create"),
        Commands::Copy => println!("Commands::Copy"),
        Commands::Combine => println!("Commands::Combine"),
        Commands::Delete => println!("Commands::Delete")
    }
    Ok(())
}
