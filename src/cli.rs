use clap::{Parser, Subcommand};
use fini::create_file;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a new file with or without content
    Create { name_of_file: Option<String>, content: Option<String> },
}

pub(crate) fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Create { name_of_file, content } => {
            create_file::run(
                name_of_file.clone().unwrap_or_else(|| "".to_string()),
                content.clone().unwrap_or_else(|| "".to_string()))
                .expect("Error running create_file command");
        }
        _ => println!("Command not supported")
    }
}