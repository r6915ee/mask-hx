use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::process::Command;

use crate::{config, fetcher};

pub fn haxe(args: Vec<String>) -> Result<(), Error> {
    match fetcher::is_config_version_installed() {
        Ok(installed) => match installed {
            true => match config::read() {
                Ok(data) => match fetcher::haxe_path(data.as_str()) {
                    Ok(buf) => {
                        let mut buf: PathBuf = buf;
                        buf.push("haxe");

                        println!("{}", buf.display());

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
