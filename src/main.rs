use clap::Parser;
use color_eyre::eyre::Result;
use command::Cli;

mod command;
mod parser;

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    cli.process()?;

    Ok(())
}
