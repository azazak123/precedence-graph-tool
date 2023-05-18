use std::{fmt::Debug, iter};

use crate::parser::GraphParser;
use color_eyre::{eyre::Context, Result};
use tabled::{
    builder::Builder,
    settings::{Alignment, Style},
    Table,
};

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
        Algorithm::Cg => graph.gc_schedule(profile),
    };

    let table = create_schedule_table(schedule);

    println!("Schedule using algorithm {}:\n{}", algorithm, table);

    Ok(())
}

fn create_schedule_table(schedule: Vec<Vec<Option<u128>>>) -> Table {
    let mut builder = Builder::default();

    for slot in schedule {
        let slot_string: Vec<String> = iter::once(&Some(slot.len() as u128))
            .chain(slot.iter())
            .map(|v| match v {
                Some(j) => j.to_string(),
                None => "-".to_string(),
            })
            .collect();

        builder.push_record(slot_string);
    }

    let mut table = builder.index().column(0).name(None).transpose().build();
    table.with(Style::modern()).with(Alignment::center());

    table
}
