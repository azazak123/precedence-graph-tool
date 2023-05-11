use std::fmt::Debug;

use crate::parser::GraphParser;
use color_eyre::{eyre::Context, Result};

use super::Algorithm;

pub fn get_schedule<'a, Parser>(
    source: Parser::Source<'a>,
    algorithm: &Algorithm,
    profile: &[usize],
) -> Result<()>
where
    Parser: GraphParser,
    Parser::Source<'a>: Debug + Clone,
{
    let graph = Parser::parse(source.clone())
        .wrap_err_with(|| format!("Failed to get graph from {:?}", source))?;

    let schedule = match algorithm {
        Algorithm::Msf => graph.msf_schedule(profile),
        Algorithm::Gc => graph.gc_schedule(profile),
    };

    println!("Schedule using algorithm {}: {:?}", algorithm, schedule);

    Ok(())
}
