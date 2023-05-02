use std::collections::HashSet;

use color_eyre::{
    eyre::{Context, ContextCompat},
    Result,
};
use itertools::Itertools;
use precedence_graph::precedence_graph::PrecedenceGraph;
use thiserror::Error;

use super::GraphParser;

#[derive(Error, Debug)]
pub enum StringParserError {
    #[error("Failed to parse: {str:?}")]
    FailedToParse { str: String },
    #[error("Node {node:?} is not a natural number")]
    NodeIsNotNumber { node: String },
    #[error("Graph {graph:?} can not be created")]
    GraphCanNotBeCreated { graph: String },
}

pub struct StringParser {}

impl GraphParser for StringParser {
    type Source<'a> = &'a str;

    fn parse(source: Self::Source<'_>) -> Result<PrecedenceGraph> {
        let content = source.chars().filter(|c| *c != ' ').collect::<String>();

        let entities = content.split_terminator(')');

        let n = entities.clone().count();

        let mut nodes = HashSet::with_capacity(n);
        let mut edges = HashSet::with_capacity(n);

        for tup in entities {
            let (outcome, income) = &tup[1..].split(',').collect_tuple().wrap_err_with(|| {
                StringParserError::FailedToParse {
                    str: tup.to_string(),
                }
            })?;

            let outcome =
                outcome
                    .parse::<u128>()
                    .wrap_err_with(|| StringParserError::NodeIsNotNumber {
                        node: outcome.to_string(),
                    })?;

            let income =
                income
                    .parse::<u128>()
                    .wrap_err_with(|| StringParserError::NodeIsNotNumber {
                        node: income.to_string(),
                    })?;

            nodes.insert(income);
            nodes.insert(outcome);

            edges.insert((outcome, income));
        }

        PrecedenceGraph::new(nodes, edges).wrap_err_with(|| {
            StringParserError::GraphCanNotBeCreated {
                graph: source.to_string(),
            }
        })
    }
}
