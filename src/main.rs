//! A program made to handle [Haxe](https://haxe.org) versions.
//!
//! `mask-hx` aims to simplify [Haxe](https://haxe.org) version
//! management. [Haxe](https://haxe.org), unlike most other toolchains,
//! does not play well with projects as a result of its
//! versioning mechanism including syntax changes. `mask-hx`
//! aims to simplify the process of version management with
//! [Haxe](https://haxe.org).

use std::io::Error;
use std::process;

use clap::{Parser, Subcommand};

use libmask::OutputLevel;
use libmask::fetcher;

/// Defines global command line flags.
///
/// The respective documentation comments for each argument below
/// is displayed using [clap].
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Activate quiet output
    #[arg(short, long)]
    quiet: bool,

    /// Activate verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Specifies a subcommand.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Defines subcommands.
///
/// The respective documentation comments for each subcommand and its respective
/// arguments are displayed using [clap].
#[derive(Subcommand)]
enum Commands {
    /// Switch between Haxe versions
    ///
    /// This creates a .mask file if it isn't present and
    /// changes it to specify a valid Haxe version. If the specified
    /// Haxe version isn't installed, then `mask-hx` will install it.
    ///
    /// If the Haxe version specified isn't valid, then the subcommand
    /// will fail.
    Switch {
        /// The Haxe version to switch to
        haxe_version: String,
    },
}

/// Defines the final output of `mask-hx`.
struct CommandResult {
    /// The message to print when `mask-hx` finishes.
    message: String,
    /// The exit status code of `mask-hx`.
    code: i32,
}

/// The entry point of the program.
///
/// This handles the arguments, as well as how the program should exit.
fn main() {
    let cli = Cli::parse();
    let output_level: OutputLevel = match cli.verbose as i8 - cli.quiet as i8 {
        -1 => OutputLevel::Quiet,
        0 => OutputLevel::Normal,
        1 => OutputLevel::Verbose,
        _ => OutputLevel::Quiet,
    };
    let result: CommandResult;

    match &cli.command {
        Some(Commands::Switch { haxe_version }) => {
            let haxe_version_valid: Result<bool, Error> =
                fetcher::is_haxe_version_valid(output_level, haxe_version);

            match haxe_version_valid {
                Ok(check) => {
                    result = CommandResult {
                        message: format!("{}", check),
                        code: 0,
                    }
                }
                Err(e) => {
                    result = CommandResult {
                        message: format!("bad tom error: {}", e),
                        code: 1,
                    }
                }
            }
        }
        None => {
            result = CommandResult {
                message: String::from(
                    "Invalid subcommand; use 'mask help' or 'mask --help' to see a list of commands",
                ),
                code: 22,
            }
        }
    }

    println!("mask: {}", result.message);

    process::exit(result.code);
}
