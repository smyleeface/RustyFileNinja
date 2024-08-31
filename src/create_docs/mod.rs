use std::fs;
use std::io::{self};
use clap::CommandFactory;
use clap_mangen::Man;
use crate::cli;

/// Begins create man docs
pub fn run() -> io::Result<()> {
    fs::create_dir_all("./docs/man")?;
    let out_dir = std::path::PathBuf::from(String::from("./docs/man/"));
    let cli_app = cli::Cli::command();
    let man = Man::new(cli_app);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    fs::write(out_dir.join("file-ninja.1"), buffer)?;
    Ok(())
}
