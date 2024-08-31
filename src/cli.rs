use clap::{Parser, Subcommand};
use fini::{create_file, copy_file, combine_files};

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
    // Copies a file to another location when given both the source and destination
    Copy { source_file: Option<String>, destination_location: Option<String> },
    /// Creates a new file with the contents of two provided files
    Combine {source_file_1: Option<String>, source_file_2: Option<String>, destination_file: Option<String> }
}

pub(crate) fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Create { name_of_file, content } => {
            create_file::run(
                name_of_file.clone().unwrap_or_else(|| "".to_string()),
                content.clone().unwrap_or_else(|| "".to_string())
            )
            .expect("Error running create_file command");
        }
        Commands::Copy { source_file, destination_location } => {
            copy_file::run(
                source_file.clone().unwrap_or_else(|| "".to_string()),
                destination_location.clone().unwrap_or_else(|| "".to_string())
            )
            .expect("Error running copy_file command")
        }
        Commands::Combine {source_file_1, source_file_2, destination_file} => {
            combine_files::run(
                source_file_1.clone().unwrap_or_else(|| "".to_string()),
                source_file_2.clone().unwrap_or_else(|| "".to_string()),
                destination_file.clone().unwrap_or_else(|| "".to_string())
            )
            .expect("Error running combine_files command");
        }
    }
}