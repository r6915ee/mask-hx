use std::ffi::OsStr;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use crate::fetcher::HaxeVersion;

/// A basic representation of a `libmask` configuration.
pub struct Config(pub HaxeVersion);

impl Config {
    /// This reads a sample configuration from the disk, and returns it if it's valid as a [Result].
    pub fn new<T: AsRef<OsStr> + Sized>(path: Option<&T>) -> Result<Config, Error> {
        let version: String = Config::read_from_file(path)?;
        Ok(Config(HaxeVersion(version)))
    }

    /// Checks a configuration path's validity and whether or not it exists, returning the path if it exists.
    ///
    /// Configuration paths are typically encased in [`Option`]s to simulate
    /// default parameters, where leaving [`None`] as the value results in the
    /// fallback path, `./.mask`, being used.
    pub fn path<T: AsRef<OsStr> + Sized>(config_location: Option<&T>) -> Result<&Path, Error> {
        let location: &str = if let Some(location) = config_location {
            location.as_ref().to_str().unwrap()
        } else {
            "./.mask"
        };
        let path: &Path = Path::new(location);
        if path.try_exists()? == true {
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
    pub fn read_from_file<T: AsRef<OsStr> + Sized>(
        supposed_path: Option<&T>,
    ) -> Result<String, Error> {
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
    pub fn write<T: AsRef<OsStr> + Sized>(self, path: Option<&T>) -> Result<(), Error> {
        fs::write(Config::path(path)?, self.0.0)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Config {
        Config(HaxeVersion("4.3.7".to_string()))
    }
}
