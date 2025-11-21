//! # libmask
//!
//! `libmask` aims to simplify [Haxe](https://haxe.org)
//! development by providing version management capabilities.
//!
//! [Haxe](https://haxe.org), unlike other toolchains and
//! programming languages, lacks unified compatibility between
//! versions; to work around this, complex systems need to
//! be set up to actually use good version management.
//!
//! `libmask` aims to provide an interface to allow programs
//! to do this easily. The de facto standard in programs that
//! use this library is `mask-hx`, which is its parent project,
//! but `libmask` is usable by anyone.

use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// Basic structure that details a Haxe version.
pub struct HaxeVersion(pub String);

impl HaxeVersion {
    /// Gets a path to this Haxe version.
    ///
    /// Do be aware that this method does **not** check whether or not the path
    /// is valid. Instead, you should use
    /// [get_path_installed](#method.get_path_installed) for this purpose,
    /// which will produce an [Error] if the path does not contain a valid Haxe
    /// installation.
    pub fn get_path(&self) -> Result<PathBuf, Error> {
        let home: Option<PathBuf> = std::env::home_dir();

        if let Some(mut buffer) = home {
            buffer.push(".haxe");
            buffer.push(&self.0);
            return Ok(buffer);
        }
        Err(Error::new(
            ErrorKind::NotFound,
            "Home directory not accessible",
        ))
    }

    /// Works the same as [get_path](#method.get_path), but returns the path to the standard library.
    pub fn get_std_path(&self) -> Result<PathBuf, Error> {
        let mut buf: PathBuf = self.get_path()?;
        buf.push("std");
        Ok(buf)
    }

    /// Checks if a Haxe version is properly installed, and returns its path if it is.
    ///
    /// This works the same as [get_path](#method.get_path), but checks for the
    /// existence of both the Haxe version and its standard library before
    /// proceeding to return the path.
    pub fn get_path_installed(&self) -> Result<PathBuf, Error> {
        if self.get_std_path()?.try_exists()? {
            Ok(self.get_path()?)
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                format!(
                    "Haxe version {} could not be found using the standard library",
                    self.0
                ),
            ))
        }
    }
}

/// A basic representation of a `libmask` configuration.
pub struct Config(pub HaxeVersion);

impl Config {
    /// This reads a sample configuration from the disk, and returns it if it's valid as a [Result].
    pub fn new(path: Option<&str>) -> Result<Config, Error> {
        let version: String = Config::read_from_file(path.unwrap_or("./.mask"))?;
        Ok(Config(HaxeVersion(version)))
    }

    /// Checks a configuration path's validity and whether or not it exists, returning the path if it exists.
    ///
    /// Configuration paths are typically encased in [`Option`]s to simulate
    /// default parameters, where leaving [`None`] as the value results in the
    /// fallback path, `./.mask`, being used.
    pub fn path(config_location: &str) -> Result<&Path, Error> {
        let path: &Path = Path::new(config_location);
        if path.try_exists()? {
            Ok(path)
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                "Configuration file's existence could not be validated",
            ))
        }
    }

    /// Reads a file from a disk, returning its contents according to
    /// [read_to_string](std::fs::read_to_string).
    pub fn read_from_file(supposed_path: &str) -> Result<String, Error> {
        match Config::path(supposed_path) {
            Ok(path) => {
                let mut contents: String = fs::read_to_string(path)?;
                contents.retain(|c| c != '\n');
                Ok(contents)
            }
            Err(e) => Err(e),
        }
    }

    /// Writes the configuration to a specified path.
    pub fn write(path: Option<&str>, version: &str) -> Result<(), Error> {
        fs::write(path.unwrap_or("./.mask"), version)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Config {
        Config(HaxeVersion("4.3.7".to_string()))
    }
}

/// Executes a specified Haxe-only program.
///
/// `libmask` will check ahead of time if the program specified is available;
/// after all, the program must be available; otherwise, this method would
/// simply not work.
pub fn haxe_exec(args: Vec<String>, config: Config, prog: Option<String>) -> Result<Output, Error> {
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
