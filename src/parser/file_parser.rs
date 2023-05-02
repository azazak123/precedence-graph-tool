use std::{
    fs,
    path::{Path, PathBuf},
};

use color_eyre::{eyre::Context, Result};
use precedence_graph::precedence_graph::PrecedenceGraph;
use thiserror::Error;

use super::{GraphParser, StringParser};

#[derive(Error, Debug)]
pub enum FileParserError {
    #[error("Failed to read file: {file:?}")]
    FailedToReadFile { file: PathBuf },
    #[error("Failed to parse: {content:?} from {file:?}")]
    FailedToParse { content: String, file: PathBuf },
}
pub struct FileParser {}

impl GraphParser for FileParser {
    type Source<'a> = &'a Path;

    fn parse(source: Self::Source<'_>) -> Result<PrecedenceGraph> {
        let content =
            fs::read_to_string(source).wrap_err_with(|| FileParserError::FailedToReadFile {
                file: source.to_path_buf(),
            })?;

        let content: String = content.chars().filter(|c| *c != '\n').collect();

        StringParser::parse(&content).wrap_err_with(|| FileParserError::FailedToParse {
            content,
            file: source.to_path_buf(),
        })
    }
}
