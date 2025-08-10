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

use libmask::fetcher;

/// Defines the "output level" of various functions.
///
/// [OutputLevel] is useful to define how the program should
/// print to the standard output. It is ignored in some cases.
#[derive(Clone)]
enum OutputLevel {
    /// Only the bare minimum will be printed.
    Quiet,
    /// Some printing will be performed. However, it doesn't expose certain information.
    Normal,
    /// Print everything that is thrown.
    Verbose,
}

/// Print to the standard output.
///
/// This macro functions identically to the [println] macro, except
/// it compares a required [OutputLevel] and the current [OutputLevel] to
/// see if the latter is greater or equal to the required output level,
/// and only printing if this comparison succeeds.
///
/// Additionally, `text` can be an expression. This is useful for concatenation
/// reasons, but more importantly, that means that the [format] macro can
/// be used as the value.
///
/// # Examples
///
/// ```
/// let current_level: OutputLevel = OutputLevel::Normal;
///
/// print_to_stdout!(OutputLevel::Normal, current_level, "The current output level is greater than the required output level");
/// ```
macro_rules! print_to_stdout {
    ($required_level: expr, $current_level: expr, $text: literal) => {
        if $current_level as u8 >= $required_level as u8 {
            println!("{}", $text);
        }
    };

    ($required_level: expr, $current_level: expr, $text: expr) => {
        if $current_level as u8 >= $required_level as u8 {
            println!("{}", $text);
        }
    };
}

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
    /// Check if a Haxe version exists
    ///
    /// This command is useful for making sure the `mask-hx` recognizes
    /// your Haxe versions.
    Check {
        /// The Haxe version to check
        haxe_version: String,
    },

    /// Check if a config in the working directory is valid
    ///
    /// Configurations are as simple as defining a Haxe version.
    /// Configuration files are named `.mask` files, and are
    /// typically hidden on UNIX-based systems.
    Config {},

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
        Some(Commands::Check { haxe_version }) => {
            match fetcher::is_haxe_version_valid(haxe_version) {
                Ok(check) => {
                    result = CommandResult {
                        message: match check {
                            true => {
                                format!("Haxe version {} is valid and ready to use", haxe_version)
                            }
                            false => format!("Haxe version {} is not valid", haxe_version),
                        },
                        code: 0,
                    }
                }
                Err(e) => {
                    result = CommandResult {
                        message: format!("io error: {}", e),
                        code: 1,
                    }
                }
            }
        }
        Some(Commands::Config {}) => match fetcher::is_config_valid() {
            Ok(check) => {
                result = CommandResult {
                    message: match check {
                        true => format!("configuration file is valid and ready to use"),
                        false => format!("configuration file is not valid"),
                    },
                    code: 0,
                }
            }
            Err(e) => {
                result = CommandResult {
                    message: format!("io error: {}", e),
                    code: 1,
                }
            }
        },
        Some(Commands::Switch { haxe_version }) => {
            print_to_stdout!(
                OutputLevel::Normal,
                output_level.clone(),
                format!("Switching to Haxe version {}...", haxe_version)
            );

            let haxe_version_valid: Result<bool, Error> =
                fetcher::is_haxe_version_valid(haxe_version);

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
                    "invalid subcommand; use 'mask help' or 'mask --help' to see a list of commands",
                ),
                code: 22,
            }
        }
    }

    println!("mask: {}", result.message);

    process::exit(result.code);
}
