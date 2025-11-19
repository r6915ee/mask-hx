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
            return Ok(PathBuf::new());
        }
        Err(Error::new(
            ErrorKind::NotFound,
            "Home directory not accessible",
        ))
    }

    /// Checks if a Haxe version is properly installed, and returns its path if it is.
    ///
    /// This works the same as [get_path](#method.get_path), but checks for the
    /// existence of both the Haxe version and its standard library before
    /// proceeding to return the path.
    pub fn get_path_installed(&self) -> Result<PathBuf, Error> {
        let mut path: PathBuf = self.get_path()?;
        return if let Ok(exists) = path.try_exists() {
            if exists {
                path.push("std");
                if let Ok(exists) = path.try_exists() {
                    return if exists {
                        Ok(path)
                    } else {
                        Err(Error::new(
                            ErrorKind::NotFound,
                            format!(
                                "Haxe version {} exists, but does not have a standard library",
                                self.0
                            ),
                        ))
                    };
                } else {
                    Err(Error::new(
                        ErrorKind::NotFound,
                        format!(
                            "Haxe version {} could be validated for existence, but not its standard library",
                            self.0
                        ),
                    ))
                }
            } else {
                Err(Error::new(
                    ErrorKind::NotFound,
                    format!("Haxe version {} was not found", self.0),
                ))
            }
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                format!(
                    "Haxe version {} could not be validated for existence",
                    self.0
                ),
            ))
        };
    }
}
