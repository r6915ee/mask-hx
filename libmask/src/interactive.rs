use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::process::Command;

use crate::{config, fetcher};

/// Executes the [Haxe](https://haxe.org) compiler and
/// passes arguments to it.
///
/// This will check if the [Haxe](https://haxe.org) compiler
/// is available for the current configuration; if it is,
/// then the Haxe compiler will be executed with the arguments
/// provided.
pub fn haxe(args: Vec<String>) -> Result<(), Error> {
    match fetcher::is_config_version_installed() {
        Ok(installed) => match installed {
            true => match config::read() {
                Ok(data) => match fetcher::haxe_path(data.as_str()) {
                    Ok(buf) => {
                        let mut buf: PathBuf = buf;
                        buf.push("haxe");

                        let mut command: Command = Command::new(buf);
                        command.args(args);
                        command.output()?;

                        Ok(())
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
            false => Err(Error::new(ErrorKind::InvalidData, "Haxe is not installed")),
        },
        Err(e) => Err(e),
    }
}
