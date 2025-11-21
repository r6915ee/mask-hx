use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use crate::fetcher::HaxeVersion;

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
    pub fn write(&self, path: Option<&str>, version: &str) -> Result<(), Error> {
        fs::write(path.unwrap_or("./.mask"), version)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Config {
        Config(HaxeVersion("4.3.7".to_string()))
    }
}
