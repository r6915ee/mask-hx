use std::io::{Error, ErrorKind};
use std::process::{Command, ExitStatus, Output, Stdio};

use crate::{config::Config, fetcher};

/// Executes a specified program
///
/// This will check if the program exists; if it does,
/// then the program will be executed with the arguments
/// provided.
pub fn exec(args: Vec<String>, config: Config, prog: Option<String>) -> Result<ExitStatus, Error> {
    let used_version: String;

    todo!();
}
