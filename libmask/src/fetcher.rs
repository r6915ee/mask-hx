use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use crate::config;

/// Checks if a Haxe version is installed.
///
/// Haxe versions are installed in the `~/.haxe/` directory, where
/// `~` is defined as the home directory as in UNIX terms.
/// The version number will be searched for in the `~/.haxe/`
/// directory, and if it's found as a directory and the standard
/// library is found under there, then [fetcher](..) will consider it to
/// be valid.
pub fn is_haxe_version_valid(version: &str) -> Result<bool, Error> {
    let home: Option<PathBuf> = std::env::home_dir();

    match home {
        Some(buf_val) => {
            let mut buf: PathBuf = buf_val.clone();

            buf.push(".haxe");
            buf.push(version);
            buf.push("std");

            buf.try_exists()
        }
        None => Err(Error::new(
            ErrorKind::NotFound,
            "Home directory not accessible",
        )),
    }
}

/// Checks if a configuration file's Haxe version is valid.
///
/// This acts the same as [is_haxe_version_valid], but instead
/// reads the version from a configuration file using [config].
pub fn is_config_version_valid() -> Result<bool, Error> {
    match config::read() {
        Ok(config) => {
            let mut config: String = config;
            config.retain(|c| c != '\n');

            is_haxe_version_valid(config.as_str())
        }
        Err(e) => Err(e),
    }
}
