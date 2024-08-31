use std::io::{self};
use fini::cli;

fn main() -> io::Result<()> {
    cli::main();
    Ok(())
}
