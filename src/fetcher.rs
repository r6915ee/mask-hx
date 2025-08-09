use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use crate::OutputLevel;

/// Push to a [PathBuf], and then print it verbosely.
macro_rules! path_buf_build {
    ($buf: expr, $output_level: expr, $path: literal) => {
        $buf.push($path);
        print_to_stdout!(OutputLevel::Verbose, $output_level, format!("{:?}", $buf));
    };

    ($buf: expr, $output_level: expr, $path: expr) => {
        $buf.push($path);
        print_to_stdout!(OutputLevel::Verbose, $output_level, format!("{:?}", $buf));
    };
}

/// Checks if a Haxe version is installed.
///
/// Haxe versions are installed in the `~/.haxe/` directory, where
/// `~` is defined as the home directory as in UNIX terms.
pub fn is_haxe_version_valid(output_level: OutputLevel, version: &str) -> Result<bool, Error> {
    print_to_stdout!(
        OutputLevel::Normal,
        output_level.clone(),
        format!("Checking if Haxe version {} is installed", version)
    );

    let home: Option<PathBuf> = std::env::home_dir();

    match home {
        Some(buf_val) => {
            let mut buf: PathBuf = buf_val.clone();

            path_buf_build!(buf, output_level.clone(), ".haxe");
            path_buf_build!(buf, output_level, version);

            buf.try_exists()
        }
        None => Err(Error::new(
            ErrorKind::NotFound,
            "Home directory not accessible",
        )),
    }
}
