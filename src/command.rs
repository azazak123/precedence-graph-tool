use clap::{Parser, Subcommand};
use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use is_quasi_interval_order::is_quasi_interval_order;
use std::path::PathBuf;

use crate::parser::StringParser;

mod is_quasi_interval_order;

/// Tool for working with precedence graphs
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check that provided graph is a quasi-interval order
    #[group(required = true, multiple = false)]
    IsQuasiIntervalOrder {
        #[arg(group = "graph")]
        graph_str: Option<String>,

        #[arg(short, long, group = "graph", value_name = "FILE")]
        file: Option<PathBuf>,
    },
}

/// Processing action for each command
impl Cli {
    pub fn process(&self) -> Result<()> {
        match &self.command {
            Command::IsQuasiIntervalOrder { file, graph_str } => {
                if let Some(graph_str) = graph_str {
                    is_quasi_interval_order::<StringParser>(&graph_str[..]).wrap_err_with(|| {
                        format!("Failed to check graph {graph_str} of quasi-interval order")
                    })?;
                } else if let Some(path) = file {
                    todo!()
                } else {
                    Err(eyre!("All args are nones in IsQuasiIntervalOrder"))?;
                }
            }
        }

        Ok(())
    }
}
