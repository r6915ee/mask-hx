use std::io::{Error, ErrorKind};
use std::path::PathBuf;

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
        if self.get_std_path()?.try_exists()? == true {
            Ok(self.get_std_path()?)
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
