use std::io::{Error, ErrorKind};
use std::path::PathBuf;

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
