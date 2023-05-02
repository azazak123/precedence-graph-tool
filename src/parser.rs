use color_eyre::Result;
use precedence_graph::precedence_graph::PrecedenceGraph;

pub use string_parser::StringParser;

mod string_parser;

pub trait GraphParser {
    type Source<'a>;

    fn parse(source: Self::Source<'_>) -> Result<PrecedenceGraph>;
}
