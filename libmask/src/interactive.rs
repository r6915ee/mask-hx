use std::io;
use std::io::{Error, ErrorKind, Write};
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Output};

use crate::{config, fetcher};

/// Executes the [Haxe](https://haxe.org) compiler and
/// passes arguments to it.
///
/// This will check if the [Haxe](https://haxe.org) compiler
/// is available for the current configuration; if it is,
/// then the Haxe compiler will be executed with the arguments
/// provided.
pub fn haxe(args: Vec<String>, haxe_version: Option<String>) -> io::Result<ExitStatus> {
    let used_version: String;

    if let Some(version) = haxe_version {
        used_version = version;
    } else {
        used_version = config::read()?;
    }

    match fetcher::is_haxe_version_installed(used_version.as_str()) {
        Ok(installed) => match installed {
            true => match fetcher::haxe_path(used_version.as_str()) {
                Ok(buf) => {
                    let mut buf: PathBuf = buf;
                    buf.push("haxe");

                    let output: Output = Command::new(buf).args(args).output()?;

                    io::stdout().write_all(&output.stdout)?;
                    io::stderr().write_all(&output.stderr)?;

                    Ok(output.status)
                }
                Err(e) => Err(e),
            },
            false => Err(Error::new(ErrorKind::NotFound, "Haxe is not installed")),
        },
        Err(e) => Err(e),
    }
}
