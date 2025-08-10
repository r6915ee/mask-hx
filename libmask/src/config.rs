use std::fs;
use std::io::Error;
use std::path::Path;

/// Checks if a configuration file exists and is readable,
/// and returns its path if successful.
pub fn path() -> Result<&'static Path, Error> {
    let config_path: &Path = Path::new("./.mask");

    match config_path.try_exists() {
        Ok(_) => Ok(config_path),
        Err(e) => Err(e),
    }
}

/// Read the contents of a configuration file and return it
/// if successful.
pub fn read() -> Result<String, Error> {
    match path() {
        Ok(config) => match fs::read_to_string(config) {
            Ok(contents) => {
                let mut contents = contents;
                contents.retain(|c| c != '\n');

                Ok(contents)
            }
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

/// Write a specified Haxe version to a configuration file.
pub fn write(version: String) -> Result<String, Error> {
    match path() {
        Ok(path) => match fs::write(path, &version) {
            Ok(_) => Ok(version),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
