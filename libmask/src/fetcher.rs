use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use crate::config;

/// Simply constructs a path to a [Haxe](https://haxe.org).
///
/// Paths to [Haxe](https://haxe.org) versions are passed to
/// this function. Do note that this does not check if they
/// actually exist; for this purpose, [is_haxe_version_installed]
/// uses this function to simply construct the path, and also
/// checks the existence of the path.
pub fn haxe_path(version: &str) -> Result<PathBuf, Error> {
    let home: Option<PathBuf> = std::env::home_dir();

    match home {
        Some(buf_val) => {
            let mut buf: PathBuf = buf_val.clone();

            buf.push(".haxe");
            println!("{}", version);
            buf.push(version);

            Ok(buf)
        }
        None => Err(Error::new(
            ErrorKind::NotFound,
            "Home directory not accessible",
        )),
    }
}

/// Checks if a Haxe version is installed.
///
/// Haxe versions are installed in the `~/.haxe/` directory, where
/// `~` is defined as the home directory as in UNIX terms.
/// The version number will be searched for in the `~/.haxe/`
/// directory, and if it's found as a directory and the standard
/// library is found under there, then [fetcher](..) will consider it to
/// be valid.
pub fn is_haxe_version_installed(version: &str) -> Result<bool, Error> {
    match haxe_path(version) {
        Ok(path) => match path.try_exists() {
            Ok(_) => {
                let mut path = path;
                path.push("std");
                match path.try_exists() {
                    Ok(exists) => Ok(exists),
                    Err(_) => Err(Error::new(
                        ErrorKind::NotFound,
                        format!(
                            "Standard library for Haxe version {} could not be found",
                            version
                        ),
                    )),
                }
            }
            Err(_) => Err(Error::new(
                ErrorKind::NotFound,
                format!("Haxe version {} could not be found", version),
            )),
        },
        Err(e) => Err(e),
    }
}

/// Checks if a configuration file's Haxe version is valid.
///
/// This acts the same as [is_haxe_version_installed], but instead
/// reads the version from a configuration file using [config].
pub fn is_config_version_installed() -> Result<bool, Error> {
    match config::read() {
        Ok(config) => {
            let mut config: String = config;
            config.retain(|c| c != '\n');

            is_haxe_version_installed(config.as_str())
        }
        Err(e) => Err(e),
    }
}
