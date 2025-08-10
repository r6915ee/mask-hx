use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

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

/// Checks if a configuration file is valid.
///
/// `.mask` files are the configuration files for `libmask`-based programs.
/// They are as simple as a [Haxe](https://haxe.org) version definition.
///
/// What this program does is attempt check if a configuration file can be
/// properly read, and then passes its data to [is_haxe_version_valid].
pub fn is_config_valid() -> Result<bool, Error> {
    let config_path: &Path = Path::new("./.mask");

    match config_path.try_exists() {
        Ok(_) => match fs::read_to_string(config_path) {
            Ok(config) => {
                let mut config: String = config;
                config.retain(|c| c != '\n');

                is_haxe_version_valid(config.as_str())
            }
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        },
        Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
    }
}
