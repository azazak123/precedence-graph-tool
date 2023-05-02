use std::fmt::Debug;

use crate::parser::GraphParser;
use color_eyre::{eyre::Context, Result};

pub fn is_quasi_interval_order<'a, Parser>(source: Parser::Source<'a>) -> Result<()>
where
    Parser: GraphParser,
    Parser::Source<'a>: Debug + Clone,
{
    let graph = Parser::parse(source.clone())
        .wrap_err_with(|| format!("Failed to get graph from {:?}", source))?;

    let quasi_interval_order = graph.is_quasi_interval_order();

    println!(
        "Graph is{} a quasi-interval order",
        if quasi_interval_order { "" } else { " not" }
    );

    Ok(())
}
