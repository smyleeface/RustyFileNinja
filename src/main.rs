mod cli;

use std::io::{self};

fn main() -> io::Result<()> {
    cli::main();
    Ok(())
}
