//! # `libmask`
//!
//! [Haxe]: https://haxe.org/
//!
//! `libmask` aims to simplify [Haxe] development by providing version
//! management capabilities.
//!
//! [Haxe], unlike other toolchains, lacks unified compatibility between
//! versions; even minor versions may create new non-backwards compatible
//! syntax. To work around this, complex systems need to be set up to
//! actually use good version management, and they are typically tedious to
//! use. This means that there may be issues in the long term, such as slower
//! work and much more trouble configuring multi-version setups.
//!
//! `libmask` aims to provide an interface to allow programs to perform
//! transparent version management. The de facto standard in programs that use
//! this library is `mask-hx`, which is its parent project, but `libmask` is
//! usable by anyone.
//!
//! ## Description
//!
//! `libmask` grants an interface to handle [Haxe] versions through filesystem
//! management and simple configuration files.
//!
//! ### Haxe Versions
//!
//! [Haxe] versions are provided through a simple [`HaxeVersion`] tuple struct.
//! The structure provides an implementation focused on the access of the
//! actual version directory.
//!
//! ### Configuration
//!
//! `libmask` uses a very simple configuration file format that contains only a
//! version number string. For example, below would be a valid configuration,
//! since it only contains a version number:
//!
//! ```c
//! 4.2.5
//! ```
//!
//! Newlines are always stripped when reading files.
//!
//! Configuration files are usable through the [`Config`] tuple struct, which
//! wraps a [`HaxeVersion`] tuple struct as data and provides configuration
//! file reading, writing, and parsing.
//!
//! ### Program Execution
//!
//! All programs under a valid [Haxe] version directory can be executed using
//! the [`haxe_exec`] method in the root module. This method modifies the
//! environment the child process is in, ensuring that further child processes
//! will also make use of the programs, avoiding complications with system
//! packages.
//!
//! ## Usage
//!
//! The following is a sample of working with `libmask`:
//!
//! ```rust
//! use libmask::*;
//!
//! # fn main() {
//! // The first argument in the configuration constructor can accept a
//! // custom configuration file path by wrapping it in an Option.
//! let config: Config = match Config::new(None) {
//!     Ok(data) => data,
//!     // Although it's not recommended to construct configurations
//!     // without performing any reading, the nature of tuple structs
//!     // allows this kind of construction.
//!     Err(_) => Config(HaxeVersion("4.2.5".into()))
//! };
//!
//! match haxe_exec(vec!["--help".into()], config, Some("haxe".into())) {
//!     Ok(_) => println!("Successfully ran Haxe compiler"),
//!     Err(e) => println!("{}", e),
//! }
//! # }
//! ```

use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// Basic structure that details [Haxe](https://haxe.org/) versions.
pub struct HaxeVersion(pub String);

impl HaxeVersion {
    /// Gets the directory where all Haxe versions are stored without performing any checking.
    ///
    /// Although this method is not typically used in most operations, it's useful
    /// for simple tasks like listing Haxe versions.
    pub fn get_haxe_installations() -> Result<PathBuf, Error> {
        let home: Option<PathBuf> = std::env::home_dir();
        if let Some(mut buffer) = home {
            buffer.push(".haxe");
            return Ok(buffer);
        }
        Err(Error::new(
            ErrorKind::NotFound,
            "Home directory not accessible",
        ))
    }

    /// Checks if a Haxe version exists, and returns its path.
    ///
    /// This is used internally by `libmask` for methods that cannot use `self`.
    pub fn get_version(path: &str) -> Result<PathBuf, Error> {
        let mut buffer: PathBuf = HaxeVersion::get_haxe_installations()?;
        buffer.push(path);
        Ok(buffer)
    }

    /// Gets a path to the current Haxe version.
    ///
    /// Do be aware that this method does **not** check whether or not the path
    /// is valid. Instead, you should use
    /// [get_path_installed](#method.get_path_installed) for this purpose,
    /// which will produce an [Error] if the path does not contain a valid Haxe
    /// installation.
    ///
    /// Internally, this method is the same as
    /// [get_version](#method.get_version) with the [String] in the tuple
    /// struct passed.
    pub fn get_path(&self) -> Result<PathBuf, Error> {
        HaxeVersion::get_version(&self.0)
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
        let version: String = Config::read_from_file(path.unwrap_or(".mask"))?;
        Ok(Config(HaxeVersion(version)))
    }

    /// Checks a configuration path's validity and whether or not it exists, returning the path if it exists.
    ///
    /// Configuration paths are typically encased in [`Option`]s to simulate
    /// default parameters, where leaving [`None`] as the value results in the
    /// fallback path, `.mask`, being used. This method will instead return the
    /// raw path through a [`Result`], as this method internally uses
    /// [`Path.try_exists`](Path#method.try_exists).
    pub fn path(config_location: &str) -> Result<&Path, Error> {
        let path: &Path = Path::new(config_location);
        if path.try_exists()? {
            Ok(path)
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                format!("Configuration file \"{}\" does not exist", path.display()),
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
        fs::write(path.unwrap_or(".mask"), version)?;
        Ok(())
    }

    /// Operates under the same conditions as [write](#method.write), except checking the Haxe version's existence beforehand.
    pub fn safe_write(path: Option<&str>, version: &str) -> Result<(), Error> {
        if HaxeVersion::get_version(version)?.try_exists()? {
            Config::write(path, version)
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                format!("Haxe version {} doesn't exist", version),
            ))
        }
    }
}

#[cfg(debug_assertions)]
impl Default for Config {
    fn default() -> Config {
        Config(HaxeVersion("4.3.7".to_string()))
    }
}

/// Executes a specified program under a version directory.
///
/// `libmask` will check ahead of time if the program specified is available as
/// a safety precaution, and will otherwise fail if the program cannot be
/// accessed.
///
/// Programs executed will have their `PATH` environment variable prepended
/// with the [Haxe](https://haxe.org/) version directory the program is in.
/// This is primarily useful for programs like build tools, because they will
/// typically expect, as an example, the compiler or Haxelib to be available.
/// Alongside this, all standard `stdio` streams are inherited for live input
/// and output.
pub fn haxe_exec(args: Vec<String>, config: Config, prog: Option<String>) -> Result<Output, Error> {
    match config.0.get_path_installed() {
        Ok(buf) => {
            let mut prog_buf: PathBuf = buf.clone();

            prog_buf.push(prog.unwrap_or("haxe".to_string()));
            if !prog_buf.try_exists()? {
                Err(Error::new(
                    ErrorKind::NotFound,
                    format!(
                        "Program at file location \"{}\" does not exist",
                        prog_buf.display()
                    ),
                ))
            } else {
                Ok(Command::new(prog_buf)
                    .args(args)
                    .env(
                        "PATH",
                        if cfg!(windows) {
                            format!(
                                "{};{}",
                                buf.display(),
                                env::var("PATH").unwrap_or("".to_string())
                            )
                        } else {
                            format!(
                                "{}:{}",
                                buf.display(),
                                env::var("PATH").unwrap_or("".to_string())
                            )
                        },
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
