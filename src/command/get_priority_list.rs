use std::fmt::Debug;

use crate::parser::GraphParser;
use color_eyre::{eyre::Context, Result};

use super::Algorithm;

pub fn get_priority_list<'a, Parser>(
    source: Parser::Source<'a>,
    algorithm: &Algorithm,
) -> Result<()>
where
    Parser: GraphParser,
    Parser::Source<'a>: Debug + Clone,
{
    let graph = Parser::parse(source.clone())
        .wrap_err_with(|| format!("Failed to get graph from {:?}", source))?;

    let priority_list = match algorithm {
        Algorithm::Msf => graph.msf_list(),
        Algorithm::Gc => graph.gc_list(),
    };

    println!(
        "Priority list using algorithm {}: {:?}",
        algorithm, priority_list
    );

    Ok(())
}
