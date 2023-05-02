use clap::{Parser, Subcommand};
use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use std::path::PathBuf;

use get_priority_list::get_priority_list;
use is_quasi_interval_order::is_quasi_interval_order;

use crate::{
    algorithm::Algorithm,
    parser::{FileParser, StringParser},
};

mod get_priority_list;
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
        #[arg(group = "graph", value_name = "GRAPH")]
        graph_str: Option<String>,

        #[arg(short, long, group = "graph", value_name = "FILE")]
        file: Option<PathBuf>,
    },

    /// Get priority list for provided graph using specified list scheduling algorithm
    #[group(required = true)]
    GetPriorityList {
        /// What algorithm to use
        #[arg(value_enum)]
        algorithm: Algorithm,

        #[arg(group = "graph", value_name = "GRAPH")]
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
                    is_quasi_interval_order::<StringParser>(&graph_str[..]).wrap_err_with(
                        || format!("Failed to check graph {graph_str:?} of quasi-interval order"),
                    )?;
                } else if let Some(path) = file {
                    is_quasi_interval_order::<FileParser>(path).wrap_err_with(|| {
                        format!("Failed to check graph from file {path:?} of quasi-interval order")
                    })?;
                } else {
                    Err(eyre!("All args are nones in IsQuasiIntervalOrder"))?;
                }
            }
            Command::GetPriorityList {
                graph_str,
                file,
                algorithm,
            } => {
                if let Some(graph_str) = graph_str {
                    get_priority_list::<StringParser>(&graph_str[..], algorithm).wrap_err_with(
                        || format!("Failed to generate priority list using {algorithm} for graph {graph_str:?}"),
                    )?;
                } else if let Some(path) = file {
                    get_priority_list::<FileParser>(path, algorithm).wrap_err_with(|| {
                         format!("Failed to generate priority list using {algorithm} for graph from file {path:?}")
                    })?;
                } else {
                    Err(eyre!("All graph sources are nones in GetPriorityList"))?;
                }
            }
        }

        Ok(())
    }
}
