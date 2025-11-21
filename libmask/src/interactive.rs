use std::env;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

use crate::config::Config;

/// Executes a specified program.
///
/// `libmask` will check ahead of time if the program specified is available;
/// after all, the program must be available; otherwise, this method would
/// simply not work.
pub fn exec(args: Vec<String>, config: Config, prog: Option<String>) -> Result<Output, Error> {
    match config.0.get_path_installed() {
        Ok(buf) => {
            let mut prog_buf: PathBuf = buf.clone();

            prog_buf.push(prog.unwrap_or("haxe".to_string()));
            if !prog_buf.try_exists()? {
                Err(Error::new(
                    ErrorKind::NotFound,
                    format!(
                        "Program at file location {} does not exist",
                        prog_buf.display()
                    ),
                ))
            } else {
                Ok(Command::new(prog_buf)
                    .args(args)
                    .env(
                        "PATH",
                        format!(
                            "{}:{}",
                            buf.display(),
                            env::var("PATH").unwrap_or("".to_string())
                        ),
                    )
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()?)
            }
        }
        Err(e) => Err(e),
    }
}
