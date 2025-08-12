use std::env;
use std::io;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Output, Stdio};

use crate::{config, fetcher};

/// Executes a specified program
///
/// This will check if the program exists; if it does,
/// then the program will be executed with the arguments
/// provided.
pub fn exec(
    args: Vec<String>,
    haxe_version: Option<String>,
    prog: Option<String>,
) -> io::Result<ExitStatus> {
    let used_version: String;

    if let Some(version) = haxe_version {
        used_version = version;
    } else {
        used_version = match config::read() {
            Ok(version) => version,
            Err(_) => String::from("0"),
        }
    }

    match fetcher::is_haxe_version_installed(used_version.as_str()) {
        Ok(installed) => match installed {
            true => match fetcher::haxe_path(used_version.as_str()) {
                Ok(buf) => {
                    let mut buf: PathBuf = buf;

                    match env::var("PATH") {
                        Ok(data) => unsafe {
                            env::set_var("PATH", format!("{}:{}", buf.display(), data));
                        },
                        Err(_) => {}
                    }

                    buf.push(prog.unwrap_or("haxe".to_string()));

                    let output: Output = Command::new(buf)
                        .args(args)
                        .stdout(Stdio::inherit())
                        .stdin(Stdio::inherit())
                        .output()?;

                    Ok(output.status)
                }
                Err(e) => Err(e),
            },
            false => Err(Error::new(ErrorKind::NotFound, "Haxe is not installed")),
        },
        Err(e) => Err(e),
    }
}
